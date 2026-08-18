#!/usr/bin/env python3
"""Emit rayon kernels directly from libxc's maple2c C sources.

Replaces the old three-stage pipeline
(`archive/kernels-cubecl` -> `xform.py` -> `flatten.py` -> `vnmerge.py`), which
transformed an archived CubeCL tree that no longer exists.

Why generating from C is *better*, not merely equivalent
--------------------------------------------------------

The CubeCL tree was never the real source; it was itself generated from these
same maple2c files. Everything the old pipeline had to undo was damage the
CubeCL emitter had done to get under `cubecl-macros`' memory ceiling:

* it split each function into `partN` pieces, re-deriving every shared
  intermediate in each piece (2-16x redundant arithmetic) -- `vnmerge.py`
  existed to value-number that back together;
* it fanned the pieces out across 231,749 `chunkN.rs` helper files --
  `flatten.py` existed to inline those back;
* it spilled five MGGA functionals across 39 `_pN` companion crates.

maple2c emits **one fully common-subexpression-eliminated function per
(order, spin)** with no duplication at all. Reading it directly means none of
those passes are needed and the 39 shard crates disappear: 305 crates -> 266.

What this does not change
-------------------------

The transform is still purely structural. Every expression keeps maple2c's
exact operand order and grouping, so floating-point results are unchanged --
which is what `AGENTS.md` requires of a maple2c translation. Nothing here
reassociates, factors, or reorders arithmetic.

Correctness posture
-------------------

The C vocabulary is small and closed, so this refuses to guess. After
translating an expression, every identifier left in it must be a known local,
parameter, input, constant or math-crate helper; anything else raises
`Untranslatable` and the functional is reported rather than emitted with a
silent mistranslation.

Usage:
    python3 tools/translate_rayon/from_maple.py --all
    python3 tools/translate_rayon/from_maple.py --func gga_x_pbe --dry-run
"""
from __future__ import annotations

import argparse
import json
import re
import shutil
import sys

import simd as simd_mod
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
MAPLE = REPO / "libxc-master" / "src" / "maple2c"
OUT = REPO / "crates" / "kernels-rayon"

ORDERS = ["exc", "vxc", "fxc", "kxc", "lxc"]

# (functional, order, spin) emitted as explicit `wide::f64x8` SIMD instead of
# scalar. This is an allowlist, not a policy, because a functional qualifies
# only if BOTH its speedup and its accuracy have been measured -- see the table
# in simd.py. Adding one without measuring risks a silent slowdown (the kernels
# already auto-vectorise 8-wide) or a silent accuracy loss (the derivative
# expressions amplify wide's ~1 ulp transcendentals by orders of magnitude, and
# by how much is a property of the formula).
SIMD_FUNCS = {
    # 5.06x scalar->SIMD, worst rel 2.7e-15 vs the scalar kernel, and oracle
    # parity against C libxc unchanged. This was the one functional in
    # docs/perf/vs-libxc.md that lost to libxc; six libm calls per point meant
    # LLVM would not vectorise the loop at all.
    ("lda_c_vwn", "vxc", "unpol"),
    ("lda_c_vwn", "exc", "unpol"),
    # The three most libm-heavy *routed* kernels (21, 11, 11 calls/pt in vxc
    # unpol; LLVM declines to vectorise loops with libm calls, so these ran
    # scalar). All three use only exp/ln/sqrt/cbrt-family transcendentals,
    # which `simd.py` now maps to the bit-exact `libxc_rkernel_math::simd`
    # forms — so the SIMD kernel is bit-identical to the scalar one, checked
    # by the `bench-vs-libxc` fingerprint staying put across the switch.
    ("mgga_c_tpssloc", "vxc", "unpol"),
    ("mgga_c_tpssloc", "exc", "unpol"),
    ("mgga_c_scan", "vxc", "unpol"),
    ("mgga_c_scan", "exc", "unpol"),
    ("mgga_c_rregtm", "vxc", "unpol"),
    ("mgga_c_rregtm", "exc", "unpol"),
}
SPINS = ["unpol", "pol"]


class Untranslatable(Exception):
    pass


# --------------------------------------------------------------------------
# Dimensions: elements per grid point, per family and spin.
# Mirrors libxc `util.c: internal_counters_set_{lda,gga,mgga}` and
# `crates/libxc-core/src/dims`. Used both for input indexing and for the
# output writes.
# --------------------------------------------------------------------------

def dims(fam: str, pol: bool) -> dict[str, int]:
    """Elements per grid point for every array, read out of
    `crates/libxc-core/src/dims/mod.rs`.

    These are NOT hand-derived here. They were once, and it was wrong:
    `v3sigma2lapl` polarized is `6*2 = 12` (libxc `util.c`
    `internal_counters_set_mgga`), not the 9 a plain count of index
    combinations suggests. Getting one of these wrong misaligns every
    subsequent grid point of that output and is invisible in a spot check, so
    the table is parsed from the single definition the eval layer also indexes
    with -- the two cannot drift apart.
    """
    key = (fam, pol)
    if key in _DIMS_CACHE:
        return _DIMS_CACHE[key]
    src = (REPO / "crates/libxc-core/src/dims/mod.rs").read_text()
    want = "Polarized" if pol else "Unpolarized"
    out: dict[str, int] = {}

    # `lda` -> `gga` -> `mgga` each start from the previous, so replay the
    # chain up to the family asked for.
    for f in ("lda", "gga", "mgga"):
        m = re.search(rf"pub fn {f}\(spin: Spin\) -> Self \{{(.*?)\n    \}}", src, re.S)
        if not m:
            raise Untranslatable(f"cannot find Dimensions::{f} in libxc-core")
        body = m.group(1)
        # assignments outside the spin match, e.g. `d.lapl = spin as u8;`
        def take(text: str) -> None:
            # libxc-core writes the polarized values as products that mirror
            # util.c (`d.v2sigmalapl = 3 * 2;`), so the right-hand side has to
            # be evaluated, not just parsed as an integer.
            for name, val in re.findall(r"d\.(\w+) = ([^;]+);", text):
                val = val.split("//")[0].strip()
                if val == "spin as u8":
                    out[name] = 2 if pol else 1
                elif re.fullmatch(r"[\d\s*+]+", val):
                    out[name] = eval(val)  # noqa: S307 - digits and * + only
                else:
                    raise Untranslatable(
                        f"cannot evaluate Dimensions::{f} {name} = {val!r}")

        take(body.split("match spin")[0])
        mm = re.search(rf"Spin::{want} => \{{(.*?)^            \}}", body, re.S | re.M)
        if mm:
            take(mm.group(1))
        if f == fam:
            break
    out.setdefault("zk", 1)
    _DIMS_CACHE[key] = out
    return out


_DIMS_CACHE: dict[tuple[str, bool], dict[str, int]] = {}


def dim_of(name: str, fam: str, pol: bool) -> int:
    d = dims(fam, pol)
    if name not in d:
        raise Untranslatable(f"no dimension recorded for {name!r} ({fam}, pol={pol})")
    return d[name]


# --------------------------------------------------------------------------
# Vocabulary
# --------------------------------------------------------------------------

# C name -> Rust path (imported from libxc_rkernel_math)
FUNCS = {
    "my_piecewise3": ("piecewise3", "piecewise"),
    "my_piecewise5": ("piecewise5", "piecewise"),
    "Heaviside": ("Heaviside", "piecewise"),
    "POW_1_3": ("pow_1_3", "powers"),
    "POW_2_3": ("pow_2_3", "powers"),
    "POW_4_3": ("pow_4_3", "powers"),
    "POW_5_3": ("pow_5_3", "powers"),
    "POW_7_3": ("pow_7_3", "powers"),
    "POW_3_2": ("pow_3_2", "powers"),
    "POW_1_4": ("pow_1_4", "powers"),
    "POW_2": ("pow_2", "powers"),
    "POW_3": ("pow_3", "powers"),
    "xc_E1_scaled": ("xc_e1_scaled", "expint_e1"),
    "xc_erfcx": ("xc_erfcx", "erf"),
    "xc_dilogarithm": ("xc_dilogarithm", "special"),
    "xc_mgga_x_br89_get_x": ("xc_mgga_x_br89_get_x", "br89"),
    "xc_mgga_x_mbrxc_get_x": ("xc_mgga_x_mbrxc_get_x", "mbrxc"),
    "xc_bessel_I0": ("xc_bessel_I0", "bessel"),
    "xc_bessel_I1": ("xc_bessel_I1", "bessel"),
    "xc_bessel_K0": ("xc_bessel_K0", "bessel"),
    "xc_bessel_K1": ("xc_bessel_K1", "bessel"),
    "xc_bessel_I0_scaled": ("xc_bessel_I0_scaled", "bessel"),
    "xc_bessel_I1_scaled": ("xc_bessel_I1_scaled", "bessel"),
    "xc_bessel_K0_scaled": ("xc_bessel_K0_scaled", "bessel"),
    "xc_bessel_K1_scaled": ("xc_bessel_K1_scaled", "bessel"),
    "lambert_w": ("lambert_w", "lambert_w"),
    "LambertW": ("lambert_w", "lambert_w"),
}

# C libm -> Rust inherent method form. These stay real libm calls, exactly as
# libxc makes them, so results match.
LIBM = {
    "sqrt": "f64::sqrt", "log": "f64::ln", "exp": "f64::exp",
    "atan": "f64::atan", "atan2": "f64::atan2", "fabs": "f64::abs",
    "tanh": "f64::tanh", "sinh": "f64::sinh", "cosh": "f64::cosh",
    "asinh": "f64::asinh", "acosh": "f64::acosh", "atanh": "f64::atanh",
    "sin": "f64::sin", "cos": "f64::cos", "tan": "f64::tan",
    "asin": "f64::asin", "acos": "f64::acos", "pow": "POWF",
    "erf": "erf_approx", "erfc": "erfc_approx", "cbrt": "cbrt_f64",
    "expm1": "f64::exp_m1", "log1p": "f64::ln_1p",
}
ERF_MOD = {"erf_approx": "erf", "erfc_approx": "erf", "cbrt_f64": "powers"}

# Which pre-specialised integrand each functional's `xc_integrate(funcN, ...)`
# resolves to, and the extra arguments the C integrand reads off `params`.
INTEGRATE_NAME = {
    ("gga_x_fd_lb94", "func0"): "xc_integrate_func0",
    ("gga_x_fd_lb94", "func1"): "xc_integrate_func1",
    ("lda_x_1d_soft", "func1"): "xc_integrate_lda_soft_func1",
    ("lda_x_1d_soft", "func2"): "xc_integrate_lda_soft_func2",
    ("lda_x_1d_exponential", "func1"): "xc_integrate_lda_exponential_func1",
    ("lda_x_1d_exponential", "func2"): "xc_integrate_lda_exponential_func2",
}
INTEGRATE_ARGS = {"gga_x_fd_lb94": ["param_beta"]}

# case21 b-splines take a fixed 10-coefficient control vector.
BSPLINE_COEFFS = 10

LANES_NOTE = ("Eight grid points per step; every lane runs maple2c's "
              "expression\n//! sequence in its original order.")

CONSTS = {"M_PI", "M_SQRTPI", "M_CBRTPI", "M_SQRT3", "M_CBRT2", "M_CBRT3",
          "M_CBRT4", "M_CBRT5", "M_CBRT6", "M_CBRT7", "M_CBRT9", "M_SQRT2",
          "M_C"}

INPUTS = {"lda": ["rho"], "gga": ["rho", "sigma"],
          "mgga": ["rho", "sigma", "lapl", "tau"]}

# Outputs written at each order, cumulative. Matches
# crates/libxc-reval `required_fields`.
OUT_ORDER = {
    "lda": [["zk"], ["vrho"], ["v2rho2"], ["v3rho3"], ["v4rho4"]],
    "gga": [["zk"], ["vrho", "vsigma"],
            ["v2rho2", "v2rhosigma", "v2sigma2"],
            ["v3rho3", "v3rho2sigma", "v3rhosigma2", "v3sigma3"],
            ["v4rho4", "v4rho3sigma", "v4rho2sigma2", "v4rhosigma3", "v4sigma4"]],
}

NUM = re.compile(r"(?<![\w.])(\d+\.\d*(?:[eE][+-]?\d+)?|\.\d+(?:[eE][+-]?\d+)?|\d+(?:[eE][+-]?\d+)?)")
# An identifier can never be preceded by a word character or a dot. Without
# that guard this matches the `e` in a converted literal like `8.64e-07` and
# reports it as an unknown symbol.
IDENT = re.compile(r"(?<![\w.])[A-Za-z_]\w*")


def c_number_to_rust(tok: str) -> str:
    """maple2c writes every constant in scientific form (`0.2e1`, `0.310907e-1`).
    Reparse and re-emit; Python's repr round-trips a double exactly, so the
    bits are identical however the text differs."""
    v = float(tok)
    r = repr(v)
    if "." not in r and "e" not in r and "E" not in r and "inf" not in r and "nan" not in r:
        r += ".0"
    return r


# --------------------------------------------------------------------------
# Parsing
# --------------------------------------------------------------------------

def strip_comments(s: str) -> str:
    s = re.sub(r"/\*.*?\*/", "", s, flags=re.S)
    return re.sub(r"//[^\n]*", "", s)


def split_functions(src: str) -> dict[tuple[str, str], str]:
    """Return {(order, spin): body} for every `func_<order>_<spin>` present."""
    src = strip_comments(src)
    out: dict[tuple[str, str], str] = {}
    for m in re.finditer(r"^func_(\w+?)_(unpol|pol)\s*\(", src, re.M):
        order, spin = m.group(1), m.group(2)
        if order not in ORDERS:
            continue
        # body runs from the opening brace to its matching close
        b = src.index("{", m.end())
        depth, i = 0, b
        while i < len(src):
            if src[i] == "{":
                depth += 1
            elif src[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        out[(order, spin)] = src[b + 1:i]
    return out


def statements(body: str) -> list[str]:
    """Split a function body into `;`-terminated statements, dropping the
    declarations and the params boilerplate."""
    body = re.sub(r"#\s*\w+[^\n]*", "", body)
    # Output writes are guarded:
    #     if(out->zk != NULL && (p->info->flags & XC_FLAGS_HAVE_EXC))
    #       out->zk[ip*p->dim.zk + 0] += tzk0;
    # The guard is libxc asking whether the caller supplied that buffer and
    # whether the functional advertises the derivative. Here `prepare()`
    # guarantees every buffer the requested order needs, and dispatch already
    # picked the right order, so the guard is dropped. Note `[^)]*` will not do
    # for stripping it -- the condition itself contains parentheses.
    body = re.sub(r"if\s*\((?:[^()]|\([^()]*\))*\)\s*(?=out->)", "", body)
    raw = [s.strip() for s in body.split(";")]
    out = []
    for s in raw:
        s = " ".join(s.split())
        if not s:
            continue
        if s.startswith("double ") or s.startswith("const double "):
            continue
        if "assert(" in s or s.startswith("params =") or s.endswith("*params"):
            continue
        if re.match(r"^\w+\s*\*\s*params$", s):
            continue
        out.append(s)
    return out


# --------------------------------------------------------------------------
# Expression translation
# --------------------------------------------------------------------------

class Ctx:
    def __init__(self, fam: str, pol: bool, params: list[str], func: str = ""):
        self.func = func
        self.integrate_args = INTEGRATE_ARGS.get(func, [])
        self.fam = fam
        self.pol = pol
        self.params = set(params)
        self.locals: dict[str, str] = {}      # name -> "f64" | "bool"
        self.used: set[tuple[str, str]] = set()   # (module, fn name)
        self.used_consts: set[str] = set()
        self.inputs = INPUTS[fam]


def translate_expr(expr: str, ctx: Ctx) -> str:
    """C expression -> Rust expression, structure untouched."""
    protected: list[str] = []

    def protect(text: str) -> str:
        # The placeholder must contain no digits: the numeric-literal pass
        # below runs over the whole string and would otherwise rewrite the
        # placeholder's own index into a float.
        protected.append(text)
        n = len(protected) - 1
        tag = "".join(chr(ord("A") + int(c)) for c in str(n))
        return f"\x01{tag}\x01"

    # 1. input array refs -> the loop's aliases (pol) or a direct index (unpol)
    def inp(m: re.Match) -> str:
        name, k = m.group(1), int(m.group(2))
        if name not in ctx.inputs:
            raise Untranslatable(f"input array {name!r} not in family {ctx.fam}")
        if ctx.pol:
            return protect(f"{name}{k}")
        if k != 0:
            raise Untranslatable(f"unpolarized body indexes {name}[{k}]")
        return protect(f"{name}[ip]")

    expr = re.sub(r"\b(rho|sigma|lapl|tau)\[(\d+)\]", inp, expr)

    # 2. thresholds and functional parameters
    expr = re.sub(r"\bp->dens_threshold\b", lambda m: protect("dens_threshold"), expr)
    expr = re.sub(r"\bp->zeta_threshold\b", lambda m: protect("zeta_threshold"), expr)

    # Hybrid mixing data (`p->hyb_omega`, `p->hyb_coeff`) lives on the
    # functional rather than in its params struct, but reaches the kernel the
    # same way: one scalar per element.
    def hyb(m: re.Match) -> str:
        n = f"param_hyb_{m.group(1)}_{m.group(2)}"
        if n not in ctx.params:
            raise Untranslatable(f"unknown p->hyb_{m.group(1)}[{m.group(2)}]")
        return protect(n)

    expr = re.sub(r"\bp->hyb_(\w+)\[(\d+)\]", hyb, expr)

    # C's float.h epsilon.
    expr = re.sub(r"\bDBL_EPSILON\b", lambda m: protect("f64::EPSILON"), expr)

    # `xc_integrate(funcN, NULL, 0.0, x)` passes an integrand by pointer. The
    # math crate has each integrand pre-specialised, so the pointer becomes
    # part of the callee name and the functional's own parameters are appended
    # (the C integrand reads them off `params`).
    def integrate(m: re.Match) -> str:
        which, arg = m.group(1), m.group(2)
        name = INTEGRATE_NAME.get((ctx.func, which))
        if name is None:
            raise Untranslatable(
                f"no math-crate integrand for {ctx.func} xc_integrate({which})")
        ctx.used.add(("integrate", name))
        extra = "".join(", " + q for q in ctx.integrate_args)
        return protect(f"{name}(") + arg + protect(f"{extra})")

    expr = re.sub(r"\bxc_integrate\s*\(\s*(\w+)\s*,\s*NULL\s*,\s*[0-9.e+-]+\s*,\s*([^,()]+)\)",
                  integrate, expr)

    # `xbspline(u, ider, params)` / `cbspline(...)`: the params struct becomes
    # the flattened coefficient list.
    def bspline(m: re.Match) -> str:
        kind, u, ider = m.group(1), m.group(2), m.group(3)
        coeffs = [q for q in ctx.params if q.startswith(f"param_c{kind}_")]
        if not coeffs:
            raise Untranslatable(f"no b-spline coefficients for {kind}bspline")
        ctx.used.add(("bspline", f"case21_{kind}bspline"))
        order = sorted(coeffs, key=lambda q: int(q.rsplit("_", 1)[1]))
        return protect(f"case21_{kind}bspline(") + f"{u}, {ider}" + protect(
            "".join(", " + q for q in order) + ")")

    expr = re.sub(r"\b([xc])bspline\s*\(\s*([^,]+)\s*,\s*(\d+)\s*,\s*params\s*\)",
                  bspline, expr)

    def par(m: re.Match) -> str:
        # An array-valued parameter becomes one scalar per element:
        # `params->c_ab[0]` -> `param_c_ab_0`. Passing a slice instead would
        # put a bounds check and an indirection in the innermost loop for a
        # value that is a compile-time constant at every call site.
        n = m.group(1) + "".join(f"_{g}" for g in m.groups()[1:] if g is not None)
        if f"param_{n}" not in ctx.params:
            raise Untranslatable(f"unknown functional parameter params->{n}")
        return protect(f"param_{n}")

    expr = re.sub(r"\bparams->(\w+)(?:\[(\d+)\])?(?:\[(\d+)\])?", par, expr)

    # 3. calls
    def call(m: re.Match) -> str:
        n = m.group(1)
        if n in FUNCS:
            rust, mod = FUNCS[n]
            ctx.used.add((mod, rust))
            return protect(rust) + "("
        if n in LIBM:
            rust = LIBM[n]
            if rust == "POWF":
                return protect("f64::powf") + "("
            if rust in ERF_MOD:
                ctx.used.add((ERF_MOD[rust], rust))
            return protect(rust) + "("
        return m.group(0)

    expr = re.sub(r"\b([A-Za-z_]\w*)\s*\(", call, expr)

    # 4. named constants
    def const(m: re.Match) -> str:
        n = m.group(0)
        if n in CONSTS:
            ctx.used_consts.add(n)
            return protect(n)
        return n

    expr = re.sub(r"\bM_[A-Z0-9_]+\b", const, expr)

    # 5. numeric literals. Safe now: every array index and identifier that
    #    could contain digits is behind a placeholder.
    expr = NUM.sub(lambda m: c_number_to_rust(m.group(1)), expr)

    # 6. anything still looking like an identifier must be a known local
    for m in IDENT.finditer(expr):
        n = m.group(0)
        if expr[m.start() - 1:m.start()] == "\x01":
            continue  # placeholder tag, restored below
        if n in ctx.locals or n == "ip":
            continue
        raise Untranslatable(f"unrecognised identifier {n!r}")

    for i, t in enumerate(protected):
        tag = "".join(chr(ord("A") + int(c)) for c in str(i))
        expr = expr.replace(f"\x01{tag}\x01", t)
    return expr


BOOL_TOP = re.compile(r"(?<![<>=!])(<=|>=|==|!=|<|>|&&|\|\|)(?![<>=])")


def is_bool_expr(expr: str) -> bool:
    """True if the *top level* of the expression is a comparison or logical
    connective, i.e. C stored a 0/1 in a double where Rust wants a `bool`."""
    depth = 0
    for i, ch in enumerate(expr):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif depth == 0:
            m = BOOL_TOP.match(expr, i)
            if m:
                return True
    return False


# --------------------------------------------------------------------------
# Emission
# --------------------------------------------------------------------------

def mgga_outputs() -> list[list[str]]:
    """Order-sliced MGGA output names, read from libxc-core so the two cannot
    drift apart."""
    s = (REPO / "crates/libxc-core/src/output/mod.rs").read_text()
    m = re.search(r"pub struct MggaOutput<.*?\{(.*?)\n\}", s, re.S)
    names = re.findall(r"pub (\w+): Option<", m.group(1))

    def rank(n: str) -> int:
        if n == "zk":
            return 0
        mm = re.match(r"v(\d)", n)
        return int(mm.group(1)) if mm else 1

    return [[n for n in names if rank(n) == i] for i in range(5)]


OUT_WRITE = re.compile(
    r"^out->(\w+)\[\s*ip\s*\*\s*p->dim\.\w+\s*\+\s*(\d+)\s*\]\s*\+=\s*(.+)$")
ASSIGN = re.compile(r"^(\w+)\s*=\s*(.+)$")


def emit_function(fam: str, func: str, order: str, spin: str,
                  body: str, params: list[str],
                  vxc_type: bool = False) -> tuple[str, set[str], set[str]]:
    pol = spin == "pol"
    ctx = Ctx(fam, pol, params, func)
    oi = ORDERS.index(order)
    outs = (mgga_outputs() if fam == "mgga" else OUT_ORDER[fam])
    wanted = [n for grp in outs[:oi + 1] for n in grp]
    if vxc_type:
        wanted = [n for n in wanted if n != "zk"]
    if not wanted:
        raise Untranslatable(f"{order} has no outputs for this functional type")

    lines: list[str] = []
    seen_writes: set[str] = set()
    for st in statements(body):
        m = OUT_WRITE.match(st)
        if m:
            name, k, val = m.group(1), int(m.group(2)), m.group(3)
            if name not in wanted:
                raise Untranslatable(f"{order} writes unexpected output {name}")
            rhs = translate_expr(val, ctx)
            d = dim_of(name, fam, pol)
            if d == 1:
                idx = "ip"
            elif k == 0:
                idx = f"ip * {d}"
            else:
                idx = f"ip * {d} + {k}"
            lines.append(f"        {name}[{idx}] += {rhs};")
            seen_writes.add(name)
            continue
        m = ASSIGN.match(st)
        if not m:
            raise Untranslatable(f"unparsed statement: {st[:80]!r}")
        name, val = m.group(1), m.group(2)
        rhs = translate_expr(val, ctx)
        ctx.locals[name] = "bool" if is_bool_expr(val) else "f64"
        lines.append(f"        let {name} = {rhs};")

    # Signature
    sig = [f"    {n}: &[f64]," for n in ctx.inputs]
    sig += [f"    {n}: &mut [f64]," for n in wanted]
    sig += [f"    {p}: f64," for p in params]
    sig += ["    dens_threshold: f64,", "    zeta_threshold: f64,"]

    # Loop preamble: polarized bodies address rho[0]/rho[1] etc., so the loop
    # binds those once per point instead of re-indexing.
    pre: list[str] = []
    if pol:
        # Bind every component of every input, not only the ones this body
        # happens to read. Unused ones are dead and cost nothing, and it keeps
        # the preamble identical across orders of the same functional.
        for nm in ctx.inputs:
            d = dim_of(nm, fam, pol)
            for k in range(d):
                idx = f"ip * {d}" if k == 0 else f"ip * {d} + {k}"
                pre.append(f"        let {nm}{k} = {nm}[{idx}];")

    guard = wanted[0]
    gd = dim_of(guard, fam, pol)
    bound = f"{guard}.len()" if gd == 1 else f"{guard}.len() / {gd}"

    if (func, order, spin) in SIMD_FUNCS:
        head = "\n".join([
            f"//! {func.upper()} {order} {spin} kernel — explicit SIMD.",
            "//!",
            f"//! Auto-translated from `libxc-master/src/maple2c/{fam}_exc/{func}.c`",
            "//! by tools/translate_rayon/from_maple.py, then rewritten to",
            f"//! `wide::f64x8` by simd.py. {LANES_NOTE}",
            "",
        ])
        # The scalar path appends the two thresholds to the signature after
        # the functional's own parameters; the SIMD form takes the same
        # arguments in the same order, so they have to be included here too.
        body = simd_mod.simd_body(
            [l.strip() for l in lines], ctx.inputs, wanted,
            list(params) + ["dens_threshold", "zeta_threshold"],
            f"{func}_{order}_{spin}")
        return head + body, seen_writes, set(wanted)

    src = [
        f"//! {func.upper()} {order} {spin} kernel (rayon backend).",
        "//!",
        f"//! Auto-translated from `libxc-master/src/maple2c/{fam}_exc/{func}.c`",
        "//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact",
        "//! variable names and floating-point operation order.",
        "",
        "#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]",
        "",
    ]
    if ctx.used_consts:
        src.append("use libxc_rkernel_math::constants::{%s};" % ", ".join(sorted(ctx.used_consts)))
    by_mod: dict[str, set[str]] = {}
    for mod, name in ctx.used:
        by_mod.setdefault(mod, set()).add(name)
    for mod in sorted(by_mod):
        src.append(
            f"use libxc_rkernel_math::{mod}::{{{', '.join(sorted(by_mod[mod]))}}};")
    src += [
        "",
        "#[allow(unused_variables, non_snake_case)]",
        f"pub fn {func}_{order}_{spin}(",
        *sig,
        ") {",
        f"    for ip in 0..{bound} {{",
        *pre,
        *lines,
        "    }",
        "}",
        "",
    ]
    return "\n".join(src), seen_writes, set(wanted)


# --------------------------------------------------------------------------
# Driver
# --------------------------------------------------------------------------

def family_of(path: Path) -> str:
    return path.parent.name.split("_")[0]


def is_vxc_type(path: Path) -> bool:
    """`maple2c/<fam>_vxc/` holds potential-only functionals (`gga_x_lb`,
    `lda_xc_tih`): they define no energy density, so there is no `exc` function
    and `zk` is absent from every output list."""
    return path.parent.name.endswith("_vxc")


def maple_files() -> dict[str, Path]:
    out: dict[str, Path] = {}
    for d in ("lda_exc", "lda_vxc", "gga_exc", "gga_vxc", "mgga_exc", "mgga_vxc"):
        for p in sorted((MAPLE / d).glob("*.c")):
            out[p.stem] = p
    return out


def load_params() -> dict:
    pj = REPO / "tools" / "translate_rayon" / "params.json"
    if not pj.is_file():
        print(f"missing {pj}; run extract_params.py --json {pj}", file=sys.stderr)
        sys.exit(2)
    return json.loads(pj.read_text())["resolved"]


def params_of(src: str, known: list[str] | None) -> list[str]:
    """Parameter list for the kernel signature.

    Taken from the C itself, in first-appearance order, so a functional whose
    `ext_params` defaults could not be resolved still gets a kernel -- it is
    only the *dispatch* that cannot be wired for those (see
    routing.rs UNSUPPORTED), not the kernel. When `params.json` does know the
    functional its order wins, because `gen_eval.py` passes the defaults
    positionally and the two must agree."""
    found: list[str] = []
    for m in re.finditer(r"\bparams->(\w+)(?:\[(\d+)\])?(?:\[(\d+)\])?", src):
        n = "param_" + m.group(1) + "".join(
            f"_{g}" for g in m.groups()[1:] if g is not None)
        if n not in found:
            found.append(n)
    # Hybrid mixing data is reached through `p->`, not `params->`, but is still
    # a per-functional constant the kernel takes as an argument.
    for m in re.finditer(r"\bp->hyb_(\w+)\[(\d+)\]", src):
        n = f"param_hyb_{m.group(1)}_{m.group(2)}"
        if n not in found:
            found.append(n)
    # `xbspline(u, ider, params)` hands the whole params struct to the spline,
    # so its coefficients never appear as `params->cx[k]` and have to be added
    # from the spline's own arity.
    for kind in ("x", "c"):
        if re.search(rf"\b{kind}bspline\s*\(", src):
            for k in range(BSPLINE_COEFFS):
                n = f"param_c{kind}_{k}"
                if n not in found:
                    found.append(n)
    if known is None:
        return found
    missing = [n for n in found if n not in known]
    if missing:
        raise Untranslatable(
            f"params.json is missing {missing}; its order drives the call site")
    return known


def emit_functional(func: str, path: Path, params: list[str] | None,
                    dry: bool) -> tuple[int, list[str]]:
    fam = family_of(path)
    text = path.read_text(errors="ignore")
    try:
        params = params_of(strip_comments(text), params)
    except Untranslatable as e:
        return 0, [f"{func}: {e}"]
    fns = split_functions(text)
    written, failed, cleared = 0, [], False
    crate = OUT / fam / func
    for (order, spin), body in sorted(fns.items()):
        try:
            src, _, _ = emit_function(fam, func, order, spin, body, params,
                                      is_vxc_type(path))
        except Untranslatable as e:
            failed.append(f"{func} {order}_{spin}: {e}")
            continue
        if not dry:
            if not cleared:
                # Wipe src/ rather than overwriting file by file. The previous
                # emitter split large outputs into directories (`lxc_pol/`)
                # that would otherwise sit alongside the new `lxc_pol.rs` and
                # collide as two definitions of the same module.
                if (crate / "src").is_dir():
                    shutil.rmtree(crate / "src")
                (crate / "src").mkdir(parents=True)
                cleared = True
            (crate / "src" / f"{order}_{spin}.rs").write_text(src)
        written += 1
    if written and not dry:
        mods = sorted(p.stem for p in (crate / "src").glob("*.rs") if p.stem != "lib")
        (crate / "src" / "lib.rs").write_text(
            f"//! {func.upper()} rayon kernels, generated by "
            "tools/translate_rayon/from_maple.py.\n\n"
            + "\n".join(f"pub mod {m};" for m in mods) + "\n")
        (crate / "Cargo.toml").write_text(
            f'[package]\nname = "libxc-rkernel-{func}"\nversion = "0.1.0"\n'
            'edition = "2024"\n\n[dependencies]\n'
            'libxc-rkernel-math = { path = "../../math" }\n')
    return written, failed


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--func", action="append", default=[])
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    files = maple_files()
    resolved = load_params()
    names = args.func or (sorted(files) if args.all else [])
    if not names:
        ap.error("pass --all or --func NAME")

    tot_fn, all_failed, no_params, done = 0, [], [], 0
    for func in names:
        if func not in files:
            print(f"  no maple2c source for {func}", file=sys.stderr)
            continue
        info = resolved.get(func)
        if info is None:
            no_params.append(func)
        n, failed = emit_functional(
            func, files[func], info["params"] if info else None, args.dry_run)
        tot_fn += n
        all_failed += failed
        if n:
            done += 1

    print(f"emitted {tot_fn} kernel functions across {done} functionals"
          + (" (dry run)" if args.dry_run else ""))
    if no_params:
        print(f"{len(no_params)} have unresolved ext_params defaults: kernels "
              f"emitted, dispatch stays unwired (routing.rs UNSUPPORTED)")
    if all_failed:
        print(f"\n{len(all_failed)} untranslatable:")
        for f in all_failed[:40]:
            print("   ", f)
    return 0


if __name__ == "__main__":
    sys.exit(main())
