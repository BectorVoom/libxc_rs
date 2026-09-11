#!/usr/bin/env python3
"""Fused composite kernels: one loop per libxc `xc_mix_init` composite.

Why
---

A composite such as HSE06 is `1.0*wpbeh(omega=0) - beta*wpbeh(omega_PBE) +
1.0*PBEc`. The mix layer (`libxc-eval/src/eval/mix.rs`) evaluates each
auxiliary into a leaf-sized scratch and folds it into the caller's output, so
the point is visited three times, a scratch buffer is written and read back
per auxiliary, and -- the expensive part -- the two `gga_x_wpbeh` legs each
recompute everything that does not depend on `omega`: `s`, `H(s)`, `F(s)`,
`EG(s)`, the `cbrt`s, the `exp`/`erf` of `Ga(s)`. Measured on the maple2c
body, 217 of the 488 statements of `wpbeh vxc unpol` are omega-independent.

This module emits, for a composite named in `FUSED`, one kernel per
(order, spin) whose body is the *concatenation* of the auxiliaries' maple2c
bodies, value-numbered so that any expression two legs both compute is
computed once, followed by the mix's own accumulation.

Two properties are what make this bit-identical to the mix path rather than
merely close:

1. **Sharing is by textual identity of the translated expression.** Two
   statements share a temp only when their right-hand sides are the same
   string after every leg-local name has been mapped to its canonical name.
   The same string is the same sequence of IEEE operations on the same
   operands, so the value is the same to the bit. Nothing is reassociated.

2. **The accumulation is the mix's, operation for operation.** The mix zeroes
   the output, runs auxiliary `k` into a zeroed scratch (so the scratch holds
   `0.0 + v_k`), and does `out += w_k * scratch` in auxiliary order. The fused
   kernel emits `out += w_k * (0.0 + v_k)` in the same order into the same
   zeroed output. `bench-vs-libxc` checks the two paths agree bit for bit on
   every composite case, and `libxc-eval`'s `mix` tests do the same.

Specialising a leg on a constant parameter
------------------------------------------

HSE's first leg is `wpbeh` with `_omega` pinned to `0.0` by
`hse03_set_ext_params`. In the maple2c body `omega` enters as
`t23 = p->hyb_omega[0] * t22` and everything screened by it -- `term3..5`,
`t2t9`, the `erfcx` call, the large-omega arm -- is multiplied by that zero.
libxc computes all of it and adds zero; the result is the same as not
computing it. A leg with a `bind` entry is run through `fold_stmts`, an
abstract interpretation over the C statements with four values:

    Z      a signed zero derived from the bound parameter
    K(v)   a literal constant
    Kb(b)  a comparison that folded to a constant
    U      anything else

and these rewrites, each exact under the two assumptions stated below:

    Z * x, x * Z, Z / x, -Z, sqrt(Z), cbrt(Z), Z + Z, Z - Z   ->  Z  (definition kept)
    x + Z, Z + x, x - Z                                       ->  x
    Z - x                                                     ->  -x
    Z <cmp> K, K <cmp> Z, K <cmp> K                           ->  Kb
    my_piecewise3(Kb, a, b)                                   ->  a or b
    my_piecewise5(Kb, a, c2, b, d)                            ->  a or my_piecewise3(c2, b, d)

A `Z`-valued temp keeps its original definition; it is only its *uses* in
`x + Z` and in folded conditions that disappear, and dead-code elimination
then drops whatever nothing reads any more. So a `Z` that flows somewhere
these rules do not cover (`log(Z)`, `exp(Z)`, a division by it, an output that
is entirely `Z`) is still computed at runtime exactly as libxc computes it.

The assumptions, and why they hold for HSE:

* **Every intermediate is finite at every point that reaches the output.**
  `0 * inf` is NaN, not zero. `wpbeh` caps the reduced gradient through
  `s_scaling_2` (`s <= smax = 8.57`) and floors it at `1e-15`, `aux1 >= D >
  0`, and `1/kF` is finite for any `rho > 0`, so every temp is finite above
  the density threshold; below it the point is screened before the kernel
  runs, or its output is discarded. A polarized channel with exactly zero
  density is guarded by `rho_s <= dens_threshold` in the same body, in libxc
  and here alike.
* **`x + Z -> x` and `Z - x -> -x` move nothing but the sign of a zero, and
  only when `x` is itself a zero.** `(-0.0) + (+0.0)` is `+0.0`, and
  `(+0.0) - (+0.0)` is `+0.0` where `-(+0.0)` is `-0.0`. No kernel output is
  a bare zero at a live point, so this is unobservable; the fingerprint gate
  would report it if it were not.

Neither assumption is needed for a leg without a `bind`, which is emitted
verbatim.

What is measured
----------------

`bench-vs-libxc --only hse06` / `--only pbeh` reports the composite's
fingerprint (must not move), a `fused vs mix` bitwise comparison (must be 0
differing values), and the ns/pt. `verify/tests/hse06_oracle.rs` and
`composite_oracle.rs` run through `Functional::evaluate_gga`, which takes the
fused path when one exists, so the C-libxc comparison covers it too.
"""
from __future__ import annotations

import re
from pathlib import Path

import from_maple as fm
import simd as simd_mod

# --------------------------------------------------------------------------
# What to fuse
# --------------------------------------------------------------------------

# name -> spec. `legs` are the auxiliaries in `xc_mix_init` order; `bind`
# pins one of that leg's kernel parameters to a constant and specialises the
# body on it (see the module docs). `parents` are the libxc functionals whose
# init/setter build exactly this mix; gen_eval.py routes them here and the
# runtime dispatch re-checks the auxiliary ids, the bound values and the
# thresholds before using the fused kernel, falling back to the mix otherwise.
FUSED: dict[str, dict] = {
    # hyb_gga_xc_hse.c: hyb_gga_xc_hse_init + hse03_set_ext_params.
    #   funcs {WPBEH, WPBEH, C_PBE}, coef {1.0, -beta, 1.0},
    #   func_aux[0]._omega = 0.0, func_aux[1]._omega = omega_PBE.
    "fused_hse": {
        "fam": "gga",
        "legs": [
            {"func": "gga_x_wpbeh", "bind": {"param_hyb_omega_0": 0.0}},
            {"func": "gga_x_wpbeh", "bind": {}},
            {"func": "gga_c_pbe", "bind": {}},
        ],
        # kxc/lxc stay on the mix path: `gga_x_wpbeh lxc_pol` alone is a 4 MB
        # body that cannot be compiled as SIMD on a 30 GB box (AGENTS.md).
        "orders": ["exc", "vxc", "fxc"],
        "parents": ["hyb_gga_xc_hse03", "hyb_gga_xc_hse06",
                    "hyb_gga_xc_hse12", "hyb_gga_xc_hse12s"],
    },
    # hyb_gga_xc_pbeh.c: hyb_gga_xc_pbeh_init + pbeh_set_ext_params.
    #   funcs {X_PBE, C_PBE}, coef {1 - beta, 1.0}.
    "fused_pbeh": {
        "fam": "gga",
        "legs": [
            {"func": "gga_x_pbe", "bind": {}},
            {"func": "gga_c_pbe", "bind": {}},
        ],
        "orders": ["exc", "vxc", "fxc"],
        "parents": ["hyb_gga_xc_pbeh", "hyb_gga_xc_pbe50",
                    "hyb_gga_xc_pbe0_13"],
    },
}


def leg_params(spec: dict, resolved: dict) -> list[list[tuple[str, float | None]]]:
    """Per leg, the kernel parameters in kernel order with their bound value
    (`None` when the parameter is taken at runtime)."""
    out = []
    for leg in spec["legs"]:
        params = resolved[leg["func"]]["params"]
        unknown = set(leg["bind"]) - set(params)
        if unknown:
            raise fm.Untranslatable(
                f"{leg['func']}: bind names {sorted(unknown)} are not kernel params")
        out.append([(p, leg["bind"].get(p)) for p in params])
    return out


def fused_scalars(spec: dict, resolved: dict) -> list[str]:
    """The scalar arguments of a fused kernel, in signature order: one weight
    per leg, then each leg's unbound parameters as `l<k>_<param>`, then **one
    `dens_threshold` per leg**, then the shared `zeta_threshold`.
    gen_eval.py builds the call site from this same list.

    The density threshold is per leg because libxc's is: `xc_mix_init` calls
    `xc_func_init` on each auxiliary, so each keeps its own
    `info->dens_threshold` and therefore its own screen and its own guards.
    HSE06's legs are `gga_x_wpbeh` twice at 1e-14 and `gga_c_pbe` at 1e-12;
    PBE0's are `gga_x_pbe` at 1e-15 and `gga_c_pbe` at 1e-12. One shared
    threshold gave two of the three legs a screen libxc does not use.

    `zeta_threshold` stays shared: libxc sets it to `DBL_EPSILON` for every
    functional and `xc_func_set_zeta_threshold` recurses into the auxiliaries,
    so the legs cannot disagree unless a caller forces them to -- which
    `crate::screen::fused_legs_agree` checks for and refuses.
    """
    names = [f"w{k}" for k in range(len(spec["legs"]))]
    for k, plist in enumerate(leg_params(spec, resolved)):
        names += [f"l{k}_{p}" for p, bound in plist if bound is None]
    names += [f"dens_threshold_{k}" for k in range(len(spec["legs"]))]
    return names + ["zeta_threshold"]


# --------------------------------------------------------------------------
# C expression parser (maple2c's dialect only)
# --------------------------------------------------------------------------

_TOK = re.compile(
    r"\s*(?:(\d+\.\d*(?:[eE][+-]?\d+)?|\.\d+(?:[eE][+-]?\d+)?|\d+(?:[eE][+-]?\d+)?)"
    r"|([A-Za-z_]\w*)"
    r"|(->|<=|>=|==|!=|&&|\|\||[-+*/<>()\[\],!]))")


def tokenize(s: str) -> list[tuple[str, str]]:
    out, i = [], 0
    while i < len(s):
        if s[i:].strip() == "":
            break
        m = _TOK.match(s, i)
        if not m or m.end() == i:
            raise fm.Untranslatable(f"cannot tokenize {s[i:i+30]!r}")
        i = m.end()
        if m.group(1) is not None:
            out.append(("num", m.group(1)))
        elif m.group(2) is not None:
            out.append(("id", m.group(2)))
        else:
            out.append(("op", m.group(3)))
    return out


class _Parser:
    """Recursive descent over C precedence: `||` < `&&` < comparisons < `+ -`
    < `* /` < unary < postfix (`[]`, `->`) < primary. Parentheses are kept as
    explicit nodes so the serialised text has exactly the source's grouping."""

    def __init__(self, toks):
        self.t = toks
        self.i = 0

    def peek(self):
        return self.t[self.i] if self.i < len(self.t) else (None, None)

    def take(self):
        tok = self.t[self.i]
        self.i += 1
        return tok

    def expect(self, op):
        tok = self.take()
        if tok != ("op", op):
            raise fm.Untranslatable(f"expected {op!r}, got {tok!r}")

    def expr(self):
        a = self.land()
        while self.peek() == ("op", "||"):
            self.take()
            a = ("bin", "||", a, self.land())
        return a

    def land(self):
        a = self.cmp()
        while self.peek() == ("op", "&&"):
            self.take()
            a = ("bin", "&&", a, self.cmp())
        return a

    def cmp(self):
        a = self.add()
        while self.peek()[0] == "op" and self.peek()[1] in ("<", ">", "<=", ">=", "==", "!="):
            op = self.take()[1]
            a = ("bin", op, a, self.add())
        return a

    def add(self):
        a = self.mul()
        while self.peek()[0] == "op" and self.peek()[1] in ("+", "-"):
            op = self.take()[1]
            a = ("bin", op, a, self.mul())
        return a

    def mul(self):
        a = self.unary()
        while self.peek()[0] == "op" and self.peek()[1] in ("*", "/"):
            op = self.take()[1]
            a = ("bin", op, a, self.unary())
        return a

    def unary(self):
        if self.peek()[0] == "op" and self.peek()[1] in ("-", "!"):
            op = self.take()[1]
            return ("un", op, self.unary())
        return self.postfix()

    def postfix(self):
        a = self.primary()
        while True:
            if self.peek() == ("op", "["):
                self.take()
                e = self.expr()
                self.expect("]")
                a = ("idx", a, e)
            elif self.peek() == ("op", "->"):
                self.take()
                kind, name = self.take()
                if kind != "id":
                    raise fm.Untranslatable("`->` not followed by a name")
                a = ("arrow", a, name)
            else:
                return a

    def primary(self):
        kind, text = self.take()
        if kind == "num":
            return ("num", text)
        if kind == "id":
            if self.peek() == ("op", "("):
                self.take()
                args = []
                if self.peek() != ("op", ")"):
                    args.append(self.expr())
                    while self.peek() == ("op", ","):
                        self.take()
                        args.append(self.expr())
                self.expect(")")
                return ("call", text, args)
            return ("id", text)
        if (kind, text) == ("op", "("):
            e = self.expr()
            self.expect(")")
            return ("paren", e)
        raise fm.Untranslatable(f"unexpected token {(kind, text)!r}")


def parse(s: str):
    p = _Parser(tokenize(s))
    node = p.expr()
    if p.i != len(p.t):
        raise fm.Untranslatable(f"trailing tokens in {s!r}")
    return node


def ser(n) -> str:
    k = n[0]
    if k in ("num", "id"):
        return n[1]
    if k == "call":
        return f"{n[1]}({', '.join(ser(a) for a in n[2])})"
    if k == "un":
        inner = ser(n[2])
        return f"{n[1]} {inner}" if n[2][0] == "un" else f"{n[1]}{inner}"
    if k == "bin":
        return f"{ser(n[2])} {n[1]} {ser(n[3])}"
    if k == "paren":
        return f"({ser(n[1])})"
    if k == "idx":
        return f"{ser(n[1])}[{ser(n[2])}]"
    if k == "arrow":
        return f"{ser(n[1])}->{n[2]}"
    raise AssertionError(k)


# --------------------------------------------------------------------------
# Abstract folding on a bound constant
# --------------------------------------------------------------------------

Z, U = "Z", "U"


class K:
    __slots__ = ("v",)

    def __init__(self, v: float):
        self.v = v


class Kb:
    __slots__ = ("v",)

    def __init__(self, v: bool):
        self.v = v


def _is_z(a) -> bool:
    return a is Z


# Functions that map a signed zero to a signed zero: sqrt(+-0) = +-0,
# cbrt likewise, |+-0| = +0, and the integer/rational powers are products
# and roots of the argument.
ZERO_PRESERVING = {"sqrt", "cbrt", "fabs", "POW_1_3", "POW_2_3", "POW_4_3",
                   "POW_5_3", "POW_7_3", "POW_3_2", "POW_1_4", "POW_2", "POW_3"}

_CMP = {
    "<": lambda a, b: a < b, ">": lambda a, b: a > b, "<=": lambda a, b: a <= b,
    ">=": lambda a, b: a >= b, "==": lambda a, b: a == b, "!=": lambda a, b: a != b,
}


def _param_name(node) -> str | None:
    """`params->x`, `params->x[i]`, `p->hyb_omega[0]` -> the kernel parameter
    name from_maple.py gives it; None for anything else."""
    text = ser(node)
    m = re.fullmatch(r"p->hyb_(\w+)\[(\d+)\]", text)
    if m:
        return f"param_hyb_{m.group(1)}_{m.group(2)}"
    m = re.fullmatch(r"params->(\w+)(?:\[(\d+)\])?(?:\[(\d+)\])?", text)
    if m:
        return "param_" + m.group(1) + "".join(
            f"_{g}" for g in m.groups()[1:] if g is not None)
    return None


def _atom(node):
    """Wrap a subtree so it can stand where a call stood (an operand position
    of any precedence) without changing its grouping."""
    return node if node[0] in ("num", "id", "call", "paren") else ("paren", node)


def _c_literal(v: float) -> str:
    r = repr(float(v))
    if r.startswith("-"):
        raise fm.Untranslatable(f"negative bound constant {v} is not supported")
    return r


def fold(node, env: dict, bind: dict):
    """Return (rewritten node, abstract value). See the module docs for the
    rule table and the assumptions behind it."""
    k = node[0]
    if k == "num":
        return node, K(float(node[1]))
    if k == "id":
        return node, env.get(node[1], U)
    if k in ("idx", "arrow"):
        pname = _param_name(node)
        if pname is not None and pname in bind:
            v = float(bind[pname])
            lit = ("num", _c_literal(v))
            return lit, (Z if v == 0.0 else K(v))
        return node, U
    if k == "paren":
        inner, a = fold(node[1], env, bind)
        return ("paren", inner), a
    if k == "un":
        inner, a = fold(node[2], env, bind)
        if node[1] == "-":
            if _is_z(a):
                return ("un", "-", inner), Z
            if isinstance(a, K):
                return ("un", "-", inner), K(-a.v)
            return ("un", "-", inner), U
        if isinstance(a, Kb):
            return ("un", "!", inner), Kb(not a.v)
        return ("un", "!", inner), U
    if k == "call":
        name = node[1]
        args = [fold(x, env, bind) for x in node[2]]
        nodes = [n for n, _ in args]
        abs_ = [a for _, a in args]
        if name == "my_piecewise3" and len(args) == 3:
            if isinstance(abs_[0], Kb):
                pick = 1 if abs_[0].v else 2
                return _atom(nodes[pick]), abs_[pick]
            both_z = _is_z(abs_[1]) and _is_z(abs_[2])
            return ("call", name, nodes), (Z if both_z else U)
        if name == "my_piecewise5" and len(args) == 5:
            if isinstance(abs_[0], Kb):
                if abs_[0].v:
                    return _atom(nodes[1]), abs_[1]
                # The remaining three-way choice is a piecewise3 on the
                # second condition; fold it with the same rules.
                return fold(("call", "my_piecewise3", [node[2][2], node[2][3], node[2][4]]),
                            env, bind)
            all_z = all(_is_z(abs_[i]) for i in (1, 3, 4))
            return ("call", name, nodes), (Z if all_z else U)
        if name in ZERO_PRESERVING and len(args) == 1 and _is_z(abs_[0]):
            return ("call", name, nodes), Z
        return ("call", name, nodes), U
    if k == "bin":
        op = node[1]
        ln, la = fold(node[2], env, bind)
        rn, ra = fold(node[3], env, bind)
        new = ("bin", op, ln, rn)
        if op == "*":
            return new, (Z if (_is_z(la) or _is_z(ra)) else U)
        if op == "/":
            return new, (Z if (_is_z(la) and not _is_z(ra)) else U)
        if op == "+":
            if _is_z(la) and _is_z(ra):
                return new, Z
            if _is_z(ra):
                return ln, la
            if _is_z(la):
                return rn, ra
            return new, U
        if op == "-":
            if _is_z(la) and _is_z(ra):
                return new, Z
            if _is_z(ra):
                return ln, la
            if _is_z(la):
                # (+-0) - x is -x exactly for every x but +0.0, where it is
                # +0.0 against -x's -0.0: the same sign-of-zero caveat as
                # `x + Z -> x`, and unobservable for the same reason.
                return ("un", "-", _atom(rn)), (K(-ra.v) if isinstance(ra, K) else U)
            return new, U
        if op in _CMP:
            def val(a):
                if _is_z(a):
                    return 0.0
                if isinstance(a, K):
                    return a.v
                return None
            lv, rv = val(la), val(ra)
            if lv is not None and rv is not None:
                return new, Kb(_CMP[op](lv, rv))
            return new, U
        if op in ("&&", "||"):
            if isinstance(la, Kb) and isinstance(ra, Kb):
                return new, Kb((la.v and ra.v) if op == "&&" else (la.v or ra.v))
            return new, U
        raise AssertionError(op)
    raise AssertionError(k)


def fold_stmts(stmts: list[str], bind: dict) -> list[str]:
    """Specialise a maple2c statement list on `bind` ({kernel param: value}).
    Output writes are passed through; every assignment is parsed, folded and
    re-serialised. A statement whose re-parse does not round-trip its own
    parse is a parser bug and stops the emit."""
    env: dict = {}
    out = []
    for st in stmts:
        if fm.OUT_WRITE.match(st):
            out.append(st)
            continue
        m = fm.ASSIGN.match(st)
        if not m:
            raise fm.Untranslatable(f"unparsed statement: {st[:80]!r}")
        name, val = m.group(1), m.group(2)
        ast = parse(val)
        if parse(ser(ast)) != ast:
            raise fm.Untranslatable(f"expression does not round-trip: {val!r}")
        new, a = fold(ast, env, bind)
        env[name] = a
        out.append(f"{name} = {ser(new)}")
    return out


# --------------------------------------------------------------------------
# Fusing translated bodies
# --------------------------------------------------------------------------

_IDENT = re.compile(r"(?<![\w.])[A-Za-z_]\w*")


def _subst(expr: str, ren: dict[str, str]) -> str:
    return _IDENT.sub(lambda m: ren.get(m.group(0), m.group(0)), expr)


def fuse_function(name: str, spec: dict, order: str, spin: str,
                  resolved: dict, files: dict[str, Path]) -> tuple[str, list[str]]:
    """Emit one fused kernel. Returns (rust source, scalar argument names)."""
    fam = spec["fam"]
    pol = spin == "pol"
    oi = fm.ORDERS.index(order)
    wanted = [n for grp in fm.OUT_ORDER[fam][:oi + 1] for n in grp]
    inputs = fm.INPUTS[fam]
    scalars = fused_scalars(spec, resolved)

    # The quantity libxc screens on: the *total* density, read from the
    # caller's array. Written as an indexed read so that the SIMD emitter
    # rewrites it into the same lane load the rest of the body uses.
    # Named the way the rest of the body names its inputs, so the SIMD
    # emitter's `_vec_expr` rewrites it into the same lane load: `rho0`/`rho1`
    # in the polarized bodies (bound by `pre`), a bare `rho[ip]` in the
    # unpolarized ones.
    dens_expr = "(rho0 + rho1)" if pol else "rho[ip]"

    table: dict[str, str] = {}          # canonical rhs -> canonical name
    lets: list[tuple[str, str]] = []    # (canonical name, rhs)
    accs: list[tuple[str, str, str]] = []  # (output, index expr, rhs)
    used: set[tuple[str, str]] = set()
    used_consts: set[str] = set()

    for k, (leg, plist) in enumerate(zip(spec["legs"], leg_params(spec, resolved))):
        func = leg["func"]
        text = files[func].read_text(errors="ignore")
        params = fm.params_of(fm.strip_comments(text), resolved[func]["params"])
        fns = fm.split_functions(text)
        if (order, spin) not in fns:
            raise fm.Untranslatable(f"{func} has no {order}_{spin} body")
        if fm.is_vxc_type(files[func]):
            raise fm.Untranslatable(f"{func} is potential-only; cannot fuse")
        sts = fm.statements(fns[(order, spin)])
        if leg["bind"]:
            sts = fold_stmts(sts, leg["bind"])

        ctx = fm.Ctx(fam, pol, params, func)
        # Leg-local renames: temps get a leg prefix, unbound params get the
        # fused signature's name, bound params were substituted as literals
        # by fold_stmts and can no longer appear.
        ren: dict[str, str] = {p: f"l{k}_{p}" for p, bound in plist if bound is None}
        # This leg's own `dens_threshold`, everywhere its body names one --
        # including the `rho_s <= dens_threshold` guards maple2c emits.
        ren["dens_threshold"] = f"dens_threshold_{k}"
        for st in sts:
            m = fm.OUT_WRITE.match(st)
            if m:
                oname, idx, val = m.group(1), int(m.group(2)), m.group(3)
                if oname not in wanted:
                    raise fm.Untranslatable(f"{func} {order} writes unexpected output {oname}")
                rhs = _subst(fm.translate_expr(fm.fold_consts(None, val, ctx), ctx), ren)
                d = fm.dim_of(oname, fam, pol)
                if d == 1:
                    ix = "ip"
                elif idx == 0:
                    ix = f"ip * {d}"
                else:
                    ix = f"ip * {d} + {idx}"
                # libxc screens each auxiliary at *its own* `dens_threshold`
                # (`work_gga_inc.c`, reached once per aux through
                # `xc_mix_func`), so a leg contributes nothing at a point below
                # its threshold while another leg still does. `w{k} * 0.0` --
                # not a bare `0.0` -- because that is exactly what the mix adds
                # there: the auxiliary's buffer is zero and the mix multiplies
                # it by the weight, which matters for the sign of the zero when
                # the weight is negative (HSE06's middle leg is `-beta`).
                accs.append((oname, ix,
                             f"w{k} * piecewise3({dens_expr} < dens_threshold_{k}, "
                             f"0.0, 0.0 + {rhs})"))
                used.add(("piecewise", "piecewise3"))
                continue
            m = fm.ASSIGN.match(st)
            if not m:
                raise fm.Untranslatable(f"unparsed statement: {st[:80]!r}")
            lname, val = m.group(1), m.group(2)
            val = fm.fold_consts(lname, val, ctx)
            rhs = _subst(fm.translate_expr(val, ctx), ren)
            ctx.locals[lname] = "bool" if fm.is_bool_expr(val) else "f64"
            if rhs in table:
                ren[lname] = table[rhs]
            else:
                cname = f"l{k}_{lname}"
                table[rhs] = cname
                ren[lname] = cname
                lets.append((cname, rhs))
        used |= ctx.used
        used_consts |= ctx.used_consts

    # Dead-code elimination: a specialised leg leaves definitions nothing
    # reads, and value numbering leaves aliases.
    live: set[str] = set()
    for _, _, rhs in accs:
        live |= set(_IDENT.findall(rhs))
    kept: list[tuple[str, str]] = []
    for cname, rhs in reversed(lets):
        if cname in live:
            live |= set(_IDENT.findall(rhs))
            kept.append((cname, rhs))
    kept.reverse()

    lines = [f"        let {c} = {r};" for c, r in kept]
    lines += [f"        {o}[{ix}] += {r};" for o, ix, r in accs]

    fn = f"{name}_{order}_{spin}"
    legs_doc = ", ".join(
        l["func"] + ("" if not l["bind"] else "(" + ", ".join(f"{p}={v}" for p, v in l["bind"].items()) + ")")
        for l in spec["legs"])
    header = [
        f"//! {name.upper()} {order} {spin}: fused composite kernel "
        f"({legs_doc}).",
        "//!",
        "//! GENERATED by tools/translate_rayon/fuse.py from the auxiliaries'",
        "//! maple2c bodies -- do not hand-edit. Shared subexpressions are",
        "//! value-numbered across legs and the mix accumulation",
        "//! (`out += w_k * (0.0 + v_k)`, in auxiliary order) is emitted inline, so",
        "//! the result is bit-identical to `evaluate_mixed_gga` on the same legs.",
    ]

    if (name, order, spin) in fm.SIMD_EXACT_FUNCS:
        header[0] = header[0].replace(": fused", " -- explicit SIMD (bit-exact): fused")
        in_dims = {n: fm.dim_of(n, fam, pol) for n in inputs}
        out_dims = {n: fm.dim_of(n, fam, pol) for n in wanted}
        body = simd_mod.simd_body([l.strip() for l in lines], inputs, wanted,
                                  scalars, fn, in_dims=in_dims, out_dims=out_dims)
        return "\n".join(header) + "\n\n" + body, scalars

    pre: list[str] = []
    if pol:
        for nm in inputs:
            d = fm.dim_of(nm, fam, pol)
            for j in range(d):
                ix = f"ip * {d}" if j == 0 else f"ip * {d} + {j}"
                pre.append(f"        let {nm}{j} = {nm}[{ix}];")
    guard = wanted[0]
    gd = fm.dim_of(guard, fam, pol)
    bound = f"{guard}.len()" if gd == 1 else f"{guard}.len() / {gd}"

    src = header + [
        "",
        "#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]",
        "",
        "use libxc_rkernel_math::rmath;",
    ]
    if used_consts:
        src.append("use libxc_rkernel_math::constants::{%s};" % ", ".join(sorted(used_consts)))
    by_mod: dict[str, set[str]] = {}
    for mod, fname in used:
        by_mod.setdefault(mod, set()).add(fname)
    for mod in sorted(by_mod):
        src.append(f"use libxc_rkernel_math::{mod}::{{{', '.join(sorted(by_mod[mod]))}}};")
    sig = [f"    {n}: &[f64]," for n in inputs]
    sig += [f"    {n}: &mut [f64]," for n in wanted]
    sig += [f"    {s}: f64," for s in scalars]
    src += [
        "",
        "#[allow(unused_variables, non_snake_case)]",
        f"pub fn {fn}(",
        *sig,
        ") {",
        f"    for ip in 0..{bound} {{",
        *pre,
        *lines,
        "    }",
        "}",
        "",
    ]
    return "\n".join(src), scalars


def emit_fused(name: str, dry: bool = False) -> tuple[int, list[str]]:
    """Emit every (order, spin) of one fused composite into its kernel crate."""
    spec = FUSED[name]
    resolved = fm.load_params()
    files = fm.maple_files()
    for leg in spec["legs"]:
        if leg["func"] not in resolved:
            raise fm.Untranslatable(f"{leg['func']}: no resolved params, cannot fuse")
        if leg["func"] not in files:
            raise fm.Untranslatable(f"{leg['func']}: no maple2c source")
    crate = fm.OUT / spec["fam"] / name
    written, failed = 0, []
    outputs: dict[str, str] = {}
    for order in spec["orders"]:
        for spin in fm.SPINS:
            try:
                src, _ = fuse_function(name, spec, order, spin, resolved, files)
            except fm.Untranslatable as e:
                failed.append(f"{name} {order}_{spin}: {e}")
                continue
            outputs[f"{order}_{spin}"] = src
            written += 1
    if written and not dry:
        import shutil
        if (crate / "src").is_dir():
            shutil.rmtree(crate / "src")
        (crate / "src").mkdir(parents=True)
        for mod, src in outputs.items():
            (crate / "src" / f"{mod}.rs").write_text(src)
        (crate / "src" / "lib.rs").write_text(
            f"//! {name.upper()} fused composite kernels, generated by "
            "tools/translate_rayon/fuse.py.\n\n"
            + "\n".join(f"pub mod {m};" for m in sorted(outputs)) + "\n")
        (crate / "Cargo.toml").write_text(
            f'[package]\nname = "libxc-rkernel-{name}"\nversion = "0.1.0"\n'
            'edition = "2024"\n\n[dependencies]\n'
            'libxc-rkernel-math = { path = "../../math" }\n')
    return written, failed


if __name__ == "__main__":
    import argparse
    import sys

    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--name", action="append", default=[],
                    help="fused kernel to emit (default: all in FUSED)")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()
    tot, bad = 0, []
    for nm in (args.name or sorted(FUSED)):
        n, f = emit_fused(nm, args.dry_run)
        tot += n
        bad += f
    print(f"emitted {tot} fused kernel functions" + (" (dry run)" if args.dry_run else ""))
    for f in bad:
        print("   ", f)
    sys.exit(1 if bad else 0)
