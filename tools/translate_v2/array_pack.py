#!/usr/bin/env python3
"""Array-packing helper for the kernel translators (compile-speed refactor).

The Maple2c translations emit one `let tN = expr;` binding per intermediate —
thousands of distinct scalar SSA locals in a single kernel/chunk body. rustc's
liveness analysis and register allocation scale super-linearly in the number of
simultaneously-live locals, so these dense temporaries are the dominant
kernel-crate compile-time/RSS cost.

This module collapses those temporaries into a single comptime-sized scratch
array. Each packable `let tN = expr;` becomes an index assignment `t[N] = expr;`
into one `let mut t = Array::<TY>::new(LEN);`, turning N distinct locals into N
element accesses of one variable.

cubecl 0.10 note: the frontend `LocalArray` type of 0.9 was REMOVED. The local
scratch-array idiom in 0.10 is `Array::<T>::new(#[comptime] len)` with index
assignment via the `ListMut` impl (cubecl-core `frontend/container/array`).
`TY` is the concrete `f64` in the flat kernels and the generic `F` inside the
`<F: Float>` CSE chunk bodies (`F: Float: ... : CubePrimitive`, so
`Array::<F>::new` is valid).

Packing rules (the task contract):
  * Only PURE-NUMERIC temporaries (`^t\\d+$`) are candidates. Named outputs
    (`tzk0`, `tvrho0`, ...) are never packed.
  * A temporary that is EXCLUDED is never packed:
      - flat path: the output-write vars (kept as `let` so the `+= var` write
        and the wrapper signature are unchanged);
      - chunk path: the tuple-INPUT params (they arrive as fn args, not slots).
  * A BOOLEAN temporary (`tN = tM <= thresh` — a top-level comparison with no
    ternary, the exact `cse._is_bool_rhs` heuristic) stays an individual
    `let tN = ...;` binding: an f64/F array cannot hold a bool.
  * Indices are DENSE and per-body: the packed temps are numbered `0..k-1` in
    first-definition order, so the array length is exactly the number of packed
    temps. This matters most on the CHUNK path, where the Maple numbers are
    GLOBAL and sparse — a late chunk that defines only `t1898`/`t1901` would
    otherwise allocate `Array::new(1902)` for two live values. Dense indexing
    sizes that chunk's array at 2. (The trade-off is that the emitted index no
    longer equals the Maple2c number; FP operation order is unaffected — only
    the storage slot changes, never the arithmetic.)

Determinism: every function here is pure in its inputs; no globals, no hashing.
The index map is keyed by first-definition order of the (already deterministic)
input line sequence.
"""

import re

from translate_v2.cse import _is_bool_rhs

# A temporary DEFINITION name: `t` followed by digits only (t1, t43, t102).
_TEMP_DEF_RE = re.compile(r'^t(\d+)$')
# A temporary REFERENCE token inside an expression. `\b` keeps it from biting
# into `tzk0`, `tau0`, `param_beta`, `M_CBRT3` (uppercase T), etc.
_TEMP_TOKEN_RE = re.compile(r'\bt(\d+)\b')


def numeric_temp_index(var):
    """Maple2c index int for a `tN` temp name, else None (`tzk0`, `tvrho0`...)."""
    if var is None:
        return None
    m = _TEMP_DEF_RE.match(var)
    return int(m.group(1)) if m else None


def is_bool_rhs(c_expr):
    """Reuse the CSE bool heuristic verbatim so the packer and the partitioner
    classify booleans identically (a bool temp must never enter the F array)."""
    return _is_bool_rhs(c_expr)


def compute_packed(parsed, exclude):
    """Decide which temporaries to pack and assign DENSE 0-based slots.

    Args:
        parsed: ordered list of ``(var, c_expr)`` for the body's compute lines,
            where ``c_expr`` is the ORIGINAL (pre-translation) C right-hand side
            so the bool heuristic sees the comparison form.
        exclude: set of var names that must NOT be packed (flat: output-write
            vars; chunk: tuple-input params).

    Returns:
        ``(index_map, length)`` — ``index_map`` maps each packed ``tN`` name to a
        dense slot ``0..length-1`` (first-definition order); ``length`` is the
        scratch-array length (the packed-temp count, or 0 if none). The dict is
        truthy iff anything is packed, so callers can gate on it directly.
    """
    index_map = {}
    for var, c_expr in parsed:
        if var in exclude:
            continue
        if numeric_temp_index(var) is None:
            continue
        if is_bool_rhs(c_expr):
            continue
        if var not in index_map:
            index_map[var] = len(index_map)
    return index_map, len(index_map)


def remap(expr, index_map):
    """Rewrite each `tN` token that is a packed temp to its dense `t[slot]`.

    Tokens NOT in ``index_map`` — bools, tuple-input params, ambient ids — are
    left untouched, so a chunk's `F` input `t10` stays `t10` while its packed
    local `t12` becomes whatever dense slot it was assigned (e.g. `t[3]`)."""
    if not index_map:
        return expr

    def _repl(m):
        name = 't' + m.group(1)
        if name in index_map:
            return f't[{index_map[name]}]'
        return m.group(0)

    return _TEMP_TOKEN_RE.sub(_repl, expr)


def decl(elem_type, length, indent):
    """The scratch-array declaration line, e.g.
    ``        let mut t = Array::<f64>::new(120usize);``

    The size is an explicit ``usize``: cubecl's ``Array::new`` takes
    ``#[comptime] length: usize``, and the ``#[cube]`` macro does NOT infer
    ``usize`` for a bare integer literal there (a bare ``new(120)`` fails E0277
    ``i32: Into<usize>``). The canonical cubecl form is ``Array::<T>::new(Nusize)``
    (cubecl-core runtime_tests/index.rs)."""
    return f'{indent}let mut t = Array::<{elem_type}>::new({length}usize);'


def emit_line(var, translated_expr, index_map, indent):
    """One emitted body line: `t[slot] = expr;` for a packed temp, else
    `let var = expr;` (bools, named outputs, chunk inputs). The literal index is
    bare — cubecl accepts bare integer literals in array-index position (`order[0]
    = 0;` in runtime_tests/index.rs). ``translated_expr`` must already be
    ``remap``-ed by the caller."""
    if var in index_map:
        return f'{indent}t[{index_map[var]}] = {translated_expr};'
    return f'{indent}let {var} = {translated_expr};'


# --- self-test ---------------------------------------------------------------
def _selftest():
    """Exercise the rules on a synthetic body. Exit 0 on success, 1 on failure."""
    ok = True

    # var, c_expr, expected-packed?
    rows = [
        ('t1', 'M_CBRT3', True),
        ('t2', '0.1e1 / M_PI', True),
        ('t33', '0.1e1 <= zeta_threshold', False),   # bool: comparison, no '?'
        ('t36', 't33 ? t34 : 0.1e1', True),          # ternary value is numeric
        ('tzk0', '-t32 + t1', False),                # named output, not tN
        ('t10', 't1 * t2', True),
    ]
    parsed = [(v, e) for v, e, _ in rows]
    index_map, length = compute_packed(parsed, exclude={'tzk0'})

    for v, _expr, want in rows:
        del _expr
        got = v in index_map
        if got != want:
            print(f"SELFTEST FAIL: {v} packed={got}, expected {want}")
            ok = False

    # Dense slots in first-definition order: t1->0, t2->1, t36->2, t10->3.
    if length != 4:
        print(f"SELFTEST FAIL: length={length}, expected 4")
        ok = False
    if index_map != {'t1': 0, 't2': 1, 't36': 2, 't10': 3}:
        print(f"SELFTEST FAIL: index_map={index_map}")
        ok = False

    # remap: packed refs -> dense t[slot]; bool ref t33 and named tzk0 stay bare.
    got = remap('t1 * t2 + t33 - tzk0 + t10', index_map)
    want = 't[0] * t[1] + t33 - tzk0 + t[3]'
    if got != want:
        print(f"SELFTEST FAIL: remap -> {got!r}, expected {want!r}")
        ok = False

    # remap must not bite into lookalikes.
    got = remap('tau0 + param_beta + M_CBRT3 + t1', {'t1': 0})
    if got != 'tau0 + param_beta + M_CBRT3 + t[0]':
        print(f"SELFTEST FAIL: lookalike remap -> {got!r}")
        ok = False

    # emit_line forms (dense slot, bare index).
    if emit_line('t1', 'M_CBRT3', index_map, '    ') != '    t[0] = M_CBRT3;':
        print("SELFTEST FAIL: packed emit_line form")
        ok = False
    if emit_line('t33', 'x <= y', index_map, '    ') != '    let t33 = x <= y;':
        print("SELFTEST FAIL: bool emit_line form")
        ok = False

    # decl: explicit usize size.
    if decl('f64', 4, '        ') != '        let mut t = Array::<f64>::new(4usize);':
        print(f"SELFTEST FAIL: decl form -> {decl('f64', 4, '        ')!r}")
        ok = False

    if ok:
        print(f"SELFTEST PASS: index_map={index_map}, length={length}")
        return 0
    return 1


if __name__ == '__main__':
    import sys
    if '--selftest' in sys.argv:
        sys.exit(_selftest())
    print("usage: python3 tools/translate_v2/array_pack.py --selftest")
    sys.exit(2)
