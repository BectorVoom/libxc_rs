#!/usr/bin/env python3
"""Rewrite an emitted scalar kernel body into explicit `wide::f64x8` SIMD.

Used by `from_maple.py` for the functionals on its `SIMD_FUNCS` allowlist.

Why an allowlist rather than "SIMD everywhere"
----------------------------------------------

The kernels already loop-vectorise 8-wide under `target-cpu=native` wherever
LLVM's cost model agrees (`docs/perf/kernel-codegen.md`), so forcing explicit
SIMD on top is only a win where LLVM *declined* -- which in practice means
kernels dominated by libm calls, because a call is what stops a loop
vectorising. Below roughly two libm calls per point the explicit form loses:
LLVM was already emitting 512-bit code and the hand-written load/store is pure
overhead. `gga_x_pbe` (0 calls) measured 0.55x and `gga_x_b88` (1 call) 0.96x;
both stand as rejections.

That is now the *only* reason to measure. There used to be a second --
accuracy -- because the transcendentals came from `wide`'s ~1 ulp forms and the
derivative expressions amplify (`gga_c_lyp` reached 4.7e-12 against its scalar
kernel, past the 1e-12 contract). Every transcendental now maps to a bit-exact
`libxc_rkernel_math::simd` form, so a SIMD kernel is bit-identical to its
scalar kernel by construction, and the gate is an unchanged `bench-vs-libxc`
fingerprint plus a measured speedup. `tools/translate_rayon/simd_qualify.py`
runs that gate and records every verdict in `docs/perf/simd-ledger.json`.

There is deliberately no approximate mode here. rmath's own free functions are
its `Fast` path and this tree called them by accident until 2026-08-31, running
4-ulp `ln` against a 1e-12 contract without any test noticing; see
`crates/kernels-rayon/math/src/rmath_bitexact.rs`. `simd_body` now refuses to
emit any call it could not map to a bit-exact form, rather than silently
falling back.

Every lane runs maple2c's expression sequence in its original order, so the
arithmetic (+,-,*,/,sqrt) is elementwise-identical to the scalar kernel, and so
now are the transcendentals.
"""
import re

LANES = 8
VT = f"f64x{LANES}"
L = LANES

# `.method()` on f64x8. Only sqrt and abs are dispatched this way, and both
# are exact in any policy, so nothing here can cost a kernel its bit-identity
# with its scalar form. Every transcendental goes through FREE_EXACT below.
UNARY_EXACT = {
    "rmath::sqrt": "sqrt", "f64::sqrt": "sqrt",
    "rmath::abs": "abs", "f64::abs": "abs",
}
# Free functions from `libxc_rkernel_math::simd` — bit-identical per lane to
# the scalar call the scalar kernel makes, because both resolve to the same
# `<BitExact, FullRange>` rmath kernel (`math/src/rmath_bitexact.rs`).
FREE_EXACT = {
    "rmath::exp": "simd::exp", "f64::exp": "simd::exp",
    "rmath::ln": "simd::ln", "f64::ln": "simd::ln",
    "rmath::cbrt": "simd::cbrt", "pow_1_3": "simd::cbrt",
    "pow_2_3": "simd::pow_2_3", "pow_4_3": "simd::pow_4_3",
    "pow_5_3": "simd::pow_5_3", "pow_7_3": "simd::pow_7_3",
    "rmath::expm1": "simd::expm1", "f64::exp_m1": "simd::expm1",
    "rmath::log1p": "simd::log1p", "f64::ln_1p": "simd::log1p",
    "rmath::atan": "simd::atan", "f64::atan": "simd::atan",
    "rmath::tanh": "simd::tanh", "f64::tanh": "simd::tanh",
    "rmath::sinh": "simd::sinh", "f64::sinh": "simd::sinh",
    "rmath::cosh": "simd::cosh", "f64::cosh": "simd::cosh",
    "rmath::asin": "simd::asin", "f64::asin": "simd::asin",
    "rmath::acos": "simd::acos", "f64::acos": "simd::acos",
    "rmath::atanh": "simd::atanh", "f64::atanh": "simd::atanh",
    "rmath::sin": "simd::sin", "f64::sin": "simd::sin",
    "rmath::cos": "simd::cos", "f64::cos": "simd::cos",
    "rmath::tan": "simd::tan", "f64::tan": "simd::tan",
    "rmath::erf": "simd::erf", "erf": "simd::erf",
    "rmath::erfc": "simd::erfc", "erfc": "simd::erfc",
    "lambert_w": "simd::lambert_w", "LambertW": "simd::lambert_w",
}
BINARY_FREE_EXACT = {
    "rmath::pow": "simd::pow", "f64::powf": "simd::pow",
    "rmath::atan2": "simd::atan2", "f64::atan2": "simd::atan2",
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


def _rewrite_one(expr):
    """Apply the first applicable call rewrite; None if nothing matched."""
    for name, tmpl in EXPAND.items():
        r = find_call(expr, name)
        if r:
            i, j, arg = r
            return expr[:i] + tmpl.format(a=rewrite_calls(arg)) + expr[j:]
    for name, fn in FREE_EXACT.items():
        r = find_call(expr, name)
        if r:
            i, j, arg = r
            return expr[:i] + f"({fn}({rewrite_calls(arg)}))" + expr[j:]
    for name, meth in UNARY_EXACT.items():
        r = find_call(expr, name)
        if r:
            i, j, arg = r
            return expr[:i] + f"(({rewrite_calls(arg)}).{meth}())" + expr[j:]
    for name, fn in BINARY_FREE_EXACT.items():
        r = find_call(expr, name)
        if r:
            i, j, args = r
            a, b = [rewrite_calls(x) for x in split_args(args)]
            return expr[:i] + f"({fn}({a}, {b}))" + expr[j:]
    # Argument 0 of piecewise3, and 0 and 2 of piecewise5, are *conditions*:
    # they may be a bare comparison, which has to become a lane mask before
    # `select` can take it.
    r = find_call(expr, "piecewise3")
    if r:
        i, j, args = r
        p = [rewrite_calls(x) for x in split_args(args)]
        p[0] = rewrite_cmp(p[0])
        return expr[:i] + f"(({p[0]}).select({p[1]}, {p[2]}))" + expr[j:]
    r = find_call(expr, "piecewise5")
    if r:
        i, j, args = r
        p = [rewrite_calls(x) for x in split_args(args)]
        p[0] = rewrite_cmp(p[0])
        p[2] = rewrite_cmp(p[2])
        return (expr[:i]
                + f"(({p[0]}).select({p[1]}, ({p[2]}).select({p[3]}, {p[4]})))"
                + expr[j:])
    r = find_call(expr, "Heaviside")
    if r:
        i, j, arg = r
        a = rewrite_calls(arg)
        return expr[:i] + f"(({a}).simd_ge(V_ZERO).select(V_ONE, V_ZERO))" + expr[j:]
    return None


def rewrite_calls(expr):
    """Map math-helper and libm calls onto f64xN forms. Recursive; leaves
    numeric/constant leaves alone (see splat_leaves)."""
    while True:
        nxt = _rewrite_one(expr)
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




def simd_body(lines, ins, outs, scalars, fn, in_dims=None, out_dims=None):
    """Turn the emitted scalar statement list into a SIMD function body."""
    if in_dims is None:
        in_dims = {n: 1 for n in ins}
    if out_dims is None:
        out_dims = {n: 1 for n in outs}

    out_lines = []
    for st in lines:
        st = st.strip()
        if not st:
            continue
        m = re.match(r"^let (\w+) = (.*);$", st)
        if m:
            e = re.sub(r"\b(rho|sigma|lapl|tau)\[ip\]", r"v_\1", m.group(2))
            e = re.sub(r"\b(rho|sigma|lapl|tau)(\d+)\b", r"v_\1\2", e)
            trans = f"            let {m.group(1)} = {splat_leaves(rewrite_cmp(rewrite_calls(e)))};"
            # Every transcendental must have been mapped to a bit-exact
            # `simd::` form above. Anything left that would evaluate
            # approximately -- one of wide's own methods, or a scalar call the
            # tables missed -- is a translation bug, not a tuning choice, so it
            # stops the emit rather than producing a kernel whose bits differ
            # from its scalar form.
            for pat, desc in [
                (r"\.powf_simd\(", "wide powf_simd"),
                (r"\.(atan|tanh|sinh|cosh|asin|acos|atanh|exp_m1|ln_1p)\(\)", "wide approximate method"),
                (r"\bf64::(exp|ln|atan|atan2|tanh|sinh|cosh|asin|acos|atanh|sin|cos|tan|erf|erfc|powf|exp_m1|ln_1p)\b", "unmapped scalar f64 transcendental"),
                (r"\brmath::(exp|ln|atan|atan2|tanh|sinh|cosh|asin|acos|atanh|sin|cos|tan|erf|erfc|pow|cbrt|expm1|log1p)\b", "unmapped scalar rmath transcendental"),
            ]:
                if re.search(pat, trans):
                    raise ValueError(f"Exact SIMD violation ({desc}) in {fn}: {trans}")
            out_lines.append(trans)
            continue
        m = re.match(r"^(\w+)\[ip\] \+= (\w+);$", st)
        if m:
            out_lines.append(f"            acc_{m.group(1)} = {m.group(2)};")
            continue
        m = re.match(r"^(\w+)\[ip \* (\d+)(?: \+ (\d+))?\] \+= (\w+);$", st)
        if m:
            k = int(m.group(3) or 0)
            out_lines.append(f"            acc_{m.group(1)}_{k} = {m.group(4)};")
            continue
        raise ValueError(f"SIMD rewrite does not handle: {st}")

    math_import = "use libxc_rkernel_math::simd;"

    has_strided = any(d > 1 for d in in_dims.values()) or any(d > 1 for d in out_dims.values())
    strided_helpers = f'''
/// Load {L} elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> {VT} {{
    let mut b = [0.0f64; {L}];
    if ip + {L} <= np {{
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    }} else {{
        for k in 0..{L} {{
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }}
    }}
    {VT}::new(b)
}}

/// Accumulate {L} elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: {VT}) {{
    let a: [f64; {L}] = acc.into();
    if m == {L} {{
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    }} else {{
        for k in 0..m {{
            s[(ip + k) * stride + offset] += a[k];
        }}
    }}
}}
''' if has_strided else ""

    # Input bindings
    in_loads = []
    for n in ins:
        d = in_dims.get(n, 1)
        if d == 1:
            in_loads.append(f"        let v_{n} = load({n}, ip, np);")
        else:
            for k in range(d):
                in_loads.append(f"        let v_{n}{k} = load_strided({n}, ip, np, {d}, {k});")

    # Output accumulators
    out_accs = []
    for n in outs:
        d = out_dims.get(n, 1)
        if d == 1:
            out_accs.append(f"        let mut acc_{n} = V_ZERO;")
        else:
            for k in range(d):
                out_accs.append(f"        let mut acc_{n}_{k} = V_ZERO;")

    # Output stores
    out_stores = []
    for n in outs:
        d = out_dims.get(n, 1)
        if d == 1:
            out_stores.append(f"        store_add({n}, ip, m, acc_{n});")
        else:
            for k in range(d):
                out_stores.append(f"        store_strided({n}, ip, m, {d}, {k}, acc_{n}_{k});")

    gd = out_dims[outs[0]]
    bound = f"{outs[0]}.len()" if gd == 1 else f"{outs[0]}.len() / {gd}"

    nl = chr(10)
    return f'''#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
{math_import}
use libxc_rkernel_math::wide::{{{VT}, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe}};

const V_ZERO: {VT} = {VT}::new([0.0; {L}]);
const V_ONE: {VT} = {VT}::new([1.0; {L}]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

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

/// Accumulate {L} consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: {VT}) {{
    let a: [f64; {L}] = acc.into();
    if m == {L} {{
        let mut b = [0.0f64; {L}];
        b.copy_from_slice(&s[ip..ip + {L}]);
        let r: [f64; {L}] = ({VT}::new(b) + acc).into();
        s[ip..ip + {L}].copy_from_slice(&r);
    }} else {{
        for k in 0..m {{
            s[ip + k] += a[k];
        }}
    }}
}}
{strided_helpers}
#[allow(unused_variables, non_snake_case)]
pub fn {fn}(
{nl.join(f"    {n}: &[f64]," for n in ins)}
{nl.join(f"    {n}: &mut [f64]," for n in outs)}
{nl.join(f"    {n}: f64," for n in scalars)}
) {{
    let np = {bound};
{nl.join(f"    let {n} = {VT}::splat({n});" for n in scalars)}
    let mut ip = 0usize;
    while ip < np {{
        let m = (np - ip).min({L});
{nl.join(in_loads)}
{nl.join(out_accs)}
        {{
{nl.join(out_lines)}
        }}
{nl.join(out_stores)}
        ip += {L};
    }}
}}
'''
