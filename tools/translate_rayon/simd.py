#!/usr/bin/env python3
"""Rewrite an emitted scalar kernel body into explicit `wide::f64x8` SIMD.

Used by `from_maple.py` for the functionals on its `SIMD_FUNCS` allowlist.

Why an allowlist rather than "SIMD everywhere"
----------------------------------------------

The kernels already loop-vectorise 8-wide under `target-cpu=native` wherever
LLVM's cost model agrees (`docs/perf/kernel-codegen.md`), so forcing explicit
SIMD on top is only a win where LLVM *declined* -- which in practice means
kernels dominated by libm calls. Measured, `vxc` unpolarized, 100k points:

    kernel        libm calls   scalar    f64x8    ratio    vs scalar kernel
    lda_c_vwn          6       68.84    13.59    5.06x     2.7e-15
    gga_c_lyp          1        9.07     8.85    1.02x     4.7e-12  <- fails tol
    gga_x_b88          1        8.85     9.20    0.96x     1.3e-15
    gga_x_pbe          0        3.98     7.25    0.55x     1.3e-15

Two independent reasons a functional must be measured before being added:

* **Speed.** Below ~2 libm calls per point the explicit form loses, because
  LLVM was already emitting 512-bit code and this adds load/store overhead on
  top.
* **Accuracy.** `wide`'s transcendentals are ~1 ulp (2.2e-16), but the
  derivative expressions amplify: `gga_c_lyp` comes out at 4.7e-12 against the
  scalar kernel, past the project's 1e-12 contract. `lda_c_vwn`, with six libm
  calls, stays at 2.7e-15. Amplification is a property of the formula, not of
  the call count, so it cannot be predicted -- only measured.

Every lane runs maple2c's expression sequence in its original order, so the
arithmetic (+,-,*,/,sqrt) is elementwise-identical to the scalar kernel; only
the transcendentals differ.
"""
import re

LANES = 8
VT = f"f64x{LANES}"
L = LANES

# `.method()` on f64x8 — wide's ~1 ulp forms. Only the calls with no
# bit-exact `simd::` replacement stay here; a kernel that uses any of the
# non-sqrt ones cannot be fingerprint-compared against its scalar form.
UNARY_EXACT = {
    "rmath::sqrt": "sqrt", "f64::sqrt": "sqrt",
    "rmath::atan": "atan", "f64::atan": "atan",
    "rmath::abs": "abs", "f64::abs": "abs",
    "rmath::tanh": "tanh", "f64::tanh": "tanh",
    "rmath::sinh": "sinh", "f64::sinh": "sinh",
    "rmath::cosh": "cosh", "f64::cosh": "cosh",
    "rmath::asin": "asin", "f64::asin": "asin",
    "rmath::acos": "acos", "f64::acos": "acos",
    "rmath::expm1": "exp_m1", "f64::exp_m1": "exp_m1",
    "rmath::log1p": "ln_1p", "f64::ln_1p": "ln_1p",
    "rmath::atanh": "atanh", "f64::atanh": "atanh",
}
# Free functions from `libxc_rkernel_math::simd` — bit-identical per lane to
# the scalar call the scalar kernel makes (exp/ln to glibc's `_fma` ifuncs,
# the cube-root family to `powers::cbrt_f64`). A kernel whose transcendentals
# all come from this set produces output bit-identical to its scalar form.
FREE_EXACT = {
    "rmath::exp": "simd::exp", "f64::exp": "simd::exp",
    "rmath::ln": "simd::ln", "f64::ln": "simd::ln",
    "rmath::cbrt": "simd::cbrt", "pow_1_3": "simd::cbrt",
    "pow_2_3": "simd::pow_2_3", "pow_4_3": "simd::pow_4_3",
    "pow_5_3": "simd::pow_5_3", "pow_7_3": "simd::pow_7_3",
}
BINARY_METHODS_EXACT = {
    "rmath::pow": "powf_simd", "f64::powf": "powf_simd",
    "rmath::atan2": "atan2", "f64::atan2": "atan2",
}

# `rmath_fast` forms for high-throughput vectorized approximation kernels.
UNARY_FAST = {
    "rmath::sqrt": "sqrt", "f64::sqrt": "sqrt",
    "rmath::abs": "abs", "f64::abs": "abs",
}
FREE_FAST = {
    "rmath::exp": "rmath_fast::exp", "f64::exp": "rmath_fast::exp",
    "rmath::ln": "rmath_fast::ln", "f64::ln": "rmath_fast::ln",
    "rmath::expm1": "rmath_fast::expm1", "f64::exp_m1": "rmath_fast::expm1",
    "rmath::log1p": "rmath_fast::log1p", "f64::ln_1p": "rmath_fast::log1p",
    "rmath::atan": "rmath_fast::atan", "f64::atan": "rmath_fast::atan",
    "rmath::tanh": "rmath_fast::tanh", "f64::tanh": "rmath_fast::tanh",
    "rmath::sinh": "rmath_fast::sinh", "f64::sinh": "rmath_fast::sinh",
    "rmath::cosh": "rmath_fast::cosh", "f64::cosh": "rmath_fast::cosh",
    "rmath::asin": "rmath_fast::asin", "f64::asin": "rmath_fast::asin",
    "rmath::acos": "rmath_fast::acos", "f64::acos": "rmath_fast::acos",
    "rmath::atanh": "rmath_fast::atanh", "f64::atanh": "rmath_fast::atanh",
    "rmath::sin": "rmath_fast::sin", "f64::sin": "rmath_fast::sin",
    "rmath::cos": "rmath_fast::cos", "f64::cos": "rmath_fast::cos",
    "rmath::tan": "rmath_fast::tan", "f64::tan": "rmath_fast::tan",
    "rmath::erf": "rmath_fast::erf", "erf": "rmath_fast::erf",
    "rmath::erfc": "rmath_fast::erfc", "erfc": "rmath_fast::erfc",
    "rmath::cbrt": "rmath_fast::cbrt", "pow_1_3": "rmath_fast::cbrt",
    "pow_2_3": "simd::pow_2_3", "pow_4_3": "simd::pow_4_3",
    "pow_5_3": "simd::pow_5_3", "pow_7_3": "simd::pow_7_3",
}
BINARY_FREE_FAST = {
    "rmath::pow": "rmath_fast::pow", "f64::powf": "rmath_fast::pow",
    "rmath::atan2": "rmath_fast::atan2", "f64::atan2": "rmath_fast::atan2",
}

CMP = {"<=": "simd_le", ">=": "simd_ge", "<": "simd_lt", ">": "simd_gt",
       "==": "simd_eq", "!=": "simd_ne"}
EXPAND = {"pow_2": "(({a}) * ({a}))",
          "pow_3": "(({a}) * ({a}) * ({a}))",
          "pow_1_4": "(({a}).sqrt().sqrt())",
          "pow_3_2": "(({a}) * ({a}).sqrt())"}


def split_args(s):
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur)
    return [a.strip() for a in out]


def find_call(expr, name):
    for m in re.finditer(r"(?<![\w:])" + re.escape(name) + r"\(", expr):
        i = m.start()
        j = m.end()
        depth = 1
        while j < len(expr) and depth:
            if expr[j] == "(":
                depth += 1
            elif expr[j] == ")":
                depth -= 1
            j += 1
        return (i, j, expr[m.end():j - 1])
    return None


def _rewrite_one(expr, math_mode="exact"):
    """Apply the first applicable call rewrite; None if nothing matched."""
    for name, tmpl in EXPAND.items():
        r = find_call(expr, name)
        if r:
            i, j, arg = r
            return expr[:i] + tmpl.format(a=rewrite_calls(arg, math_mode=math_mode)) + expr[j:]
    free_table = FREE_FAST if math_mode == "fast" else FREE_EXACT
    for name, fn in free_table.items():
        r = find_call(expr, name)
        if r:
            i, j, arg = r
            return expr[:i] + f"({fn}({rewrite_calls(arg, math_mode=math_mode)}))" + expr[j:]
    unary_table = UNARY_FAST if math_mode == "fast" else UNARY_EXACT
    for name, meth in unary_table.items():
        r = find_call(expr, name)
        if r:
            i, j, arg = r
            return expr[:i] + f"(({rewrite_calls(arg, math_mode=math_mode)}).{meth}())" + expr[j:]
    if math_mode == "fast":
        for name, fn in BINARY_FREE_FAST.items():
            r = find_call(expr, name)
            if r:
                i, j, args = r
                a, b = [rewrite_calls(x, math_mode=math_mode) for x in split_args(args)]
                return expr[:i] + f"({fn}({a}, {b}))" + expr[j:]
    else:
        for name, meth in BINARY_METHODS_EXACT.items():
            r = find_call(expr, name)
            if r:
                i, j, args = r
                a, b = [rewrite_calls(x, math_mode=math_mode) for x in split_args(args)]
                return expr[:i] + f"(({a}).{meth}({b}))" + expr[j:]
    # Argument 0 of piecewise3, and 0 and 2 of piecewise5, are *conditions*:
    # they may be a bare comparison, which has to become a lane mask before
    # `select` can take it.
    r = find_call(expr, "piecewise3")
    if r:
        i, j, args = r
        p = [rewrite_calls(x, math_mode=math_mode) for x in split_args(args)]
        p[0] = rewrite_cmp(p[0])
        return expr[:i] + f"(({p[0]}).select({p[1]}, {p[2]}))" + expr[j:]
    r = find_call(expr, "piecewise5")
    if r:
        i, j, args = r
        p = [rewrite_calls(x, math_mode=math_mode) for x in split_args(args)]
        p[0] = rewrite_cmp(p[0])
        p[2] = rewrite_cmp(p[2])
        return (expr[:i]
                + f"(({p[0]}).select({p[1]}, ({p[2]}).select({p[3]}, {p[4]})))"
                + expr[j:])
    r = find_call(expr, "Heaviside")
    if r:
        i, j, arg = r
        a = rewrite_calls(arg, math_mode=math_mode)
        return expr[:i] + f"(({a}).simd_ge(V_ZERO).select(V_ONE, V_ZERO))" + expr[j:]
    return None


def rewrite_calls(expr, math_mode="exact"):
    """Map math-helper and libm calls onto f64xN forms. Recursive; leaves
    numeric/constant leaves alone (see splat_leaves)."""
    while True:
        nxt = _rewrite_one(expr, math_mode=math_mode)
        if nxt is None:
            return expr
        expr = nxt


def rewrite_cmp(expr):
    """A top-level comparison becomes a lane mask; top-level `||`/`&&` become
    mask `|`/`&` (equivalent on all-ones/all-zero lane masks)."""
    depth = 0
    for i in range(len(expr) - 1):
        ch = expr[i]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif depth == 0 and expr[i:i + 2] in ("||", "&&"):
            op = "|" if expr[i] == "|" else "&"
            return (f"({rewrite_cmp(expr[:i].strip())}) {op} "
                    f"({rewrite_cmp(expr[i + 2:].strip())})")
    depth = 0
    for i, ch in enumerate(expr):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif depth == 0:
            for op in ("<=", ">=", "==", "!="):
                if expr.startswith(op, i):
                    return f"({expr[:i].strip()}).{CMP[op]}({expr[i+2:].strip()})"
            if expr[i] in "<>" and not expr.startswith("<=", i) and not expr.startswith(">=", i):
                return f"({expr[:i].strip()}).{CMP[expr[i]]}({expr[i+1:].strip()})"
    return expr


def splat_leaves(expr):
    """Splat f64 leaves to lanes. Runs ONCE, after all call rewriting: doing it
    inside the recursion double-splats, because the regex matches the literal
    inside a `splat(..)` it just produced."""
    expr = re.sub(r"\bM_[A-Z0-9_]+\b", lambda m: f"{VT}::splat({m.group(0)})", expr)
    # Scalar f64 associated constants (only EPSILON occurs in the tree today).
    expr = re.sub(r"\bf64::(EPSILON|MIN_POSITIVE|INFINITY|NEG_INFINITY|MAX|MIN)\b",
                  lambda m: f"{VT}::splat(f64::{m.group(1)})", expr)
    return re.sub(
        r"(?<![\w.])(\d+\.\d*(?:[eE][+-]?\d+)?|\d+(?:[eE][+-]?\d+)?)(?![\w.])",
        lambda m: "{}::splat({}{})".format(
            VT, m.group(1),
            "" if ("." in m.group(1) or "e" in m.group(1).lower()) else ".0"),
        expr)




def simd_body(lines, ins, outs, scalars, fn, math_mode="exact"):
    """Turn the emitted scalar statement list into a SIMD function body."""
    out_lines = []
    for st in lines:
        st = st.strip()
        if not st:
            continue
        m = re.match(r"^let (\w+) = (.*);$", st)
        if m:
            e = re.sub(r"\b(rho|sigma|lapl|tau)\[ip\]", r"v_\1", m.group(2))
            out_lines.append(
                f"            let {m.group(1)} = "
                f"{splat_leaves(rewrite_cmp(rewrite_calls(e, math_mode=math_mode)))};")
            continue
        m = re.match(r"^(\w+)\[ip\] \+= (\w+);$", st)
        if m:
            out_lines.append(f"            acc_{m.group(1)} = {m.group(2)};")
            continue
        raise ValueError(f"SIMD rewrite does not handle: {st}")

    math_import = ("use libxc_rkernel_math::rmath_fast;\nuse libxc_rkernel_math::simd;"
                   if math_mode == "fast" else "use libxc_rkernel_math::simd;")

    nl = chr(10)
    return f'''#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
{math_import}
use libxc_rkernel_math::wide::{{{VT}, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe}};

const V_ZERO: {VT} = {VT}::new([0.0; {L}]);
const V_ONE: {VT} = {VT}::new([1.0; {L}]);

// `exp`, `ln` and the cube-root family come from `libxc_rkernel_math::simd`,
// which is bit-identical per lane to the scalar calls the scalar kernel makes
// (exp/ln to glibc's `_fma` ifuncs, cbrt to `powers::cbrt_f64`). Only
// `atan`/`tanh`-class calls still use `wide`'s ~1 ulp forms; a kernel with
// none of those produces output bit-identical to its scalar form.

/// Load {L} consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> {VT} {{
    if ip + {L} <= np {{
        let mut b = [0.0f64; {L}];
        b.copy_from_slice(&s[ip..ip + {L}]);
        {VT}::new(b)
    }} else {{
        let mut b = [s[np - 1]; {L}];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        {VT}::new(b)
    }}
}}

#[allow(unused_variables, non_snake_case)]
pub fn {fn}(
{nl.join(f"    {n}: &[f64]," for n in ins)}
{nl.join(f"    {n}: &mut [f64]," for n in outs)}
{nl.join(f"    {n}: f64," for n in scalars)}
) {{
    let np = {outs[0]}.len();
{nl.join(f"    let {n} = {VT}::splat({n});" for n in scalars)}
    let mut ip = 0usize;
    while ip < np {{
        let m = (np - ip).min({L});
{nl.join(f"        let v_{n} = load({n}, ip, np);" for n in ins)}
{nl.join(f"        let mut acc_{n} = V_ZERO;" for n in outs)}
        {{
{nl.join(out_lines)}
        }}
{nl.join(f"        {{ let a: [f64; {L}] = acc_{n}.into(); {n}[ip..ip + m].copy_from_slice(&a[..m]); }}" for n in outs)}
        ip += {L};
    }}
}}
'''
