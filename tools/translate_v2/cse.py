#!/usr/bin/env python3
"""CSE-aware compute_lines partitioner for splitter v2 (Phase 11 D-01).

Partitions an ordered list of C-level ``tN = expr;`` compute lines into
chunks each estimated <= ``chunk_max_lines``, computing tuple in/out
signatures so each chunk can be emitted as a D-02 tuple-return
``#[cube] fn chunk<F: Float>(args: F, ...) -> (F, F, ...)`` helper.

Operates on the C ``compute_lines`` list (RESEARCH Option C) — NOT the Maple
AST, NOT a Rust ``syn`` pass. Deterministic: the same input always yields the
same chunk sequence (chunk ids are strict 0-based sequence indices; tuple
members are sorted by first-use line number).

Build env source of truth: .cargo/config.toml (do not duplicate values here).

Provenance note (D-01): ``build_dependency_graph`` and ``transitive_deps`` are
copied VERBATIM from ``tools/translate_lda_v2.py`` (the dep-graph foundation
the RESEARCH doc says to reuse). They are copied rather than imported because
the three translators import this module at their top level, and this module
importing back from ``translate_lda_v2`` would form a circular import that
fails during partial module initialization. The plan anticipates this:
"if a circular-import problem appears, copy verbatim and note it in SUMMARY".
"""

from dataclasses import dataclass
import re

# --- Splitter v2 knobs (RESEARCH "CSE Detection Heuristic") -------------------
CHUNK_MAX_LINES = 4500          # leave headroom vs the 5000 hard cap (D-LOCK-B)
MIN_REVERSE_DEPS = 5            # natural-breakpoint reverse-dep threshold
MIN_CHAIN_LENGTH = 50           # min def-use chain span before a breakpoint counts
MAX_TUPLE_ARITY = 16            # cubecl-macros tuple arity cap (drop to 8 if needed)


@dataclass
class Chunk:
    """One CSE chunk: a contiguous run of compute lines with a tuple signature."""
    index: int                  # 0-based, sequence order (deterministic chunk id)
    lines: list                 # the compute lines in this chunk, in original order
    inputs: list                # vars referenced inside but defined before (tuple args)
    outputs: list               # vars defined inside with >=1 use after (tuple returns)
    est_lines: int              # estimated emitted Rust line count


# --- Copied verbatim from tools/translate_lda_v2.py (see provenance note) -----
def build_dependency_graph(compute_lines):
    """Build a dependency graph from C compute lines."""
    var_order = []
    var_deps = {}
    for cline in compute_lines:
        stripped = cline.rstrip(';').strip()
        m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
        if not m:
            continue
        var = m.group(1)
        expr = m.group(2)
        refs = set(re.findall(r'\b(t\w+)\b', expr))
        refs.discard(var)
        refs -= {'true', 'tanh', 'tan', 'trunc'}
        var_order.append(var)
        var_deps[var] = refs
    return var_order, var_deps


def transitive_deps(variables, var_deps):
    """Compute transitive closure of dependencies for a set of variables."""
    result = set()
    stack = list(variables)
    while stack:
        v = stack.pop()
        if v in result:
            continue
        result.add(v)
        for dep in var_deps.get(v, set()):
            if dep not in result:
                stack.append(dep)
    return result
# --- end verbatim copy -------------------------------------------------------


_ASSIGN_RE = re.compile(r'^\s*(\w+)\s*=\s*(.*)$')
# Comparison operators whose presence (without a ternary `?`) marks a bool RHS.
_CMP_RE = re.compile(r'(<=|>=|==|!=|<|>)')


def _parse_assign(line):
    """Return (var, expr) for a ``tN = expr;`` line, or None for a non-assignment."""
    stripped = line.rstrip(';').strip()
    m = _ASSIGN_RE.match(stripped)
    if not m:
        return None
    return m.group(1), m.group(2)


def _is_bool_rhs(expr):
    """True when the RHS evaluates to a bool that CANNOT pass through an `F` tuple.

    Heuristic: a top-level comparison operator (`<= < >= > == !=`) with no
    ternary `?` means the result itself is a bool (e.g. `t45 = t44 <= 0.1e-14`).
    A ternary `cond ? x : y` also contains a comparison in its *condition* but
    its *value* is numeric, so `?`-bearing expressions are NOT bool. `->` is
    stripped first so `params->field` does not false-match on `>`.
    """
    cleaned = expr.replace('->', '')
    if '?' in cleaned:
        return False
    return bool(_CMP_RE.search(cleaned))


def _refs_of(expr):
    """The set of `tN` temporaries referenced in an expression (build_dependency_graph rule)."""
    refs = set(re.findall(r'\b(t\w+)\b', expr))
    refs -= {'true', 'tanh', 'tan', 'trunc'}
    return refs


def _close_chunk(index, parsed, start, end, def_index, last_use, est_lines):
    """Build a Chunk for parsed[start:end+1].

    INPUTS  = vars referenced inside the chunk but defined before `start`,
              ordered by first-use line number (deterministic).
    OUTPUTS = vars defined inside the chunk with at least one use after `end`,
              ordered by definition line number (deterministic).
    Bool vars never appear in either list — the partitioner guarantees a bool's
    definition and all its uses live in the SAME chunk (see partition docstring).
    """
    inputs_first_use = {}
    outputs = []
    defined_here = set()
    for i in range(start, end + 1):
        var = parsed[i][0]
        if var is not None:
            defined_here.add(var)
    for i in range(start, end + 1):
        var, expr, refs, is_bool, _raw = parsed[i]
        for r in refs:
            if r not in defined_here and r not in inputs_first_use:
                # referenced here, defined before this chunk -> tuple input
                if r in def_index and def_index[r] < start:
                    inputs_first_use[r] = i
        if var is not None and not is_bool:
            lu = last_use.get(var, i)
            if lu > end:
                outputs.append(var)
    inputs = sorted(inputs_first_use, key=lambda v: inputs_first_use[v])
    return Chunk(
        index=index,
        lines=[parsed[i][4] for i in range(start, end + 1)],
        inputs=inputs,
        outputs=outputs,
        est_lines=est_lines,
    )


def partition_compute_lines(compute_lines, var_deps, est_line_fn, *,
                            chunk_max_lines=CHUNK_MAX_LINES):
    """Partition ``compute_lines`` into deterministic CSE chunks.

    Walk lines in order; accumulate into the current chunk until its estimated
    size hits ``chunk_max_lines`` OR a natural breakpoint is reached (a var with
    reverse-dep-count >= MIN_REVERSE_DEPS whose def-use chain spans at least
    MIN_CHAIN_LENGTH lines and extends past the current line). The chunk is also
    closed early when its input or output tuple arity would reach
    MAX_TUPLE_ARITY — this is the "force a chunk split EARLIER" rule.

    Determinism: chunk ids are strict 0-based sequence indices (never hash- or
    set-order-derived); tuple inputs are sorted by first-use line, outputs by
    definition line. The same input list always yields the same Chunk list.

    Bool-intermediate constraint: a bool temporary (`tN = tM <= thresh`) cannot
    pass through an `F` tuple. The partitioner therefore never closes a chunk
    while a bool defined inside it still has a use further ahead — the bool's
    definition and ALL of its uses are forced into the same chunk. A bool is
    consequently never emitted as a chunk input or output.

    Args:
        compute_lines: ordered list of C ``tN = expr;`` strings.
        var_deps: dict var -> set(referenced vars), from ``build_dependency_graph``.
        est_line_fn: callable(list[str]) -> int, the caller's Rust line estimator.
        chunk_max_lines: soft per-chunk cap (default CHUNK_MAX_LINES).

    Returns:
        list[Chunk] in deterministic sequence order. A single chunk is returned
        unchanged when the whole input already fits under ``chunk_max_lines``.
    """
    # Parse every line once: (var, expr, refs, is_bool, raw_line).
    parsed = []
    for ln in compute_lines:
        pa = _parse_assign(ln)
        if pa is None:
            parsed.append((None, None, set(), False, ln))
            continue
        var, expr = pa
        refs = var_deps.get(var)
        if refs is None:
            refs = _refs_of(expr)
        else:
            refs = set(refs)
        refs.discard(var)
        parsed.append((var, expr, refs, _is_bool_rhs(expr), ln))

    n = len(parsed)
    if n == 0:
        return []

    # First definition index per var, reverse-dep counts, last-use index.
    def_index = {}
    reverse_deps = {}
    last_use = {}
    for i, (var, _expr, refs, _is_bool, _raw) in enumerate(parsed):
        if var is not None and var not in def_index:
            def_index[var] = i
        for r in refs:
            reverse_deps[r] = reverse_deps.get(r, 0) + 1
            last_use[r] = i

    chunks = []
    start = 0
    forced_until = -1            # bool constraint: cannot close before this index
    # Incremental input/output arity tracking for the current chunk.
    defined_in_chunk = set()
    inputs_seen = set()
    # last_use index -> count of in-chunk (non-bool) vars whose last use is there.
    lu_buckets = {}
    outputs_open = 0             # in-chunk vars with last_use strictly ahead of i

    def _reset_chunk_state():
        defined_in_chunk.clear()
        inputs_seen.clear()
        lu_buckets.clear()

    i = 0
    while i < n:
        var, expr, refs, is_bool, _raw = parsed[i]

        # Drop vars whose last use is exactly the previous lines (now closed).
        # outputs_open counts vars with last_use > i; recompute the boundary by
        # removing this index's bucket once we pass it (handled at chunk close).

        # Track inputs: refs to vars defined before this chunk started.
        for r in refs:
            if r not in defined_in_chunk and r not in inputs_seen:
                if r in def_index and def_index[r] < start:
                    inputs_seen.add(r)

        # Register this var's definition.
        if var is not None:
            defined_in_chunk.add(var)
            if is_bool and var in last_use:
                # bool: force its definition and all uses into this chunk.
                forced_until = max(forced_until, last_use[var])
            elif not is_bool:
                lu = last_use.get(var, i)
                if lu > i:
                    lu_buckets[lu] = lu_buckets.get(lu, 0) + 1

        # outputs_open = count of in-chunk non-bool vars with last_use > i.
        outputs_open = sum(c for lu, c in lu_buckets.items() if lu > i)
        inputs_arity = len(inputs_seen)

        cur_lines = [parsed[k][4] for k in range(start, i + 1)]
        est = est_line_fn(cur_lines)
        at_last = (i == n - 1)

        natural = False
        if var is not None:
            rd = reverse_deps.get(var, 0)
            lu = last_use.get(var, i)
            di = def_index.get(var, i)
            if rd >= MIN_REVERSE_DEPS and (lu - di) >= MIN_CHAIN_LENGTH and lu > i:
                natural = True

        arity_forced = (inputs_arity >= MAX_TUPLE_ARITY
                        or outputs_open >= MAX_TUPLE_ARITY)

        want_close = at_last or est >= chunk_max_lines or natural or arity_forced

        if want_close and i >= forced_until:
            chunks.append(_close_chunk(len(chunks), parsed, start, i,
                                       def_index, last_use, est))
            start = i + 1
            forced_until = -1
            _reset_chunk_state()
        i += 1

    # Flush any trailing lines that never tripped a close condition.
    if start < n:
        cur_lines = [parsed[k][4] for k in range(start, n)]
        chunks.append(_close_chunk(len(chunks), parsed, start, n - 1,
                                   def_index, last_use,
                                   est_line_fn(cur_lines)))
    return chunks


# --- self-test ---------------------------------------------------------------
def _selftest():
    """Run a synthetic compute_lines list through partition_compute_lines.

    Asserts the partition is deterministic across two calls and that no chunk
    exceeds chunk_max_lines. Exit 0 on success, 1 on failure.
    """
    # Synthetic kernel: a long linear chain plus one bool intermediate.
    lines = []
    lines.append("t1 = rho0 + rho1;")
    for k in range(2, 320):
        lines.append(f"t{k} = t{k-1} * 0.5 + t1;")
    # bool intermediate used a few lines later (must not cross a chunk boundary)
    lines.append("t320 = t319 <= 0.1e-14;")
    lines.append("t321 = t320 ? t1 : t319;")
    for k in range(322, 640):
        lines.append(f"t{k} = t{k-1} - t1;")

    _vo, var_deps = build_dependency_graph(lines)
    est = lambda ls: len(ls) + 25

    a = partition_compute_lines(lines, var_deps, est, chunk_max_lines=120)
    b = partition_compute_lines(lines, var_deps, est, chunk_max_lines=120)

    ok = True

    # Determinism: same chunk count, same ids, same line splits, same signatures.
    if len(a) != len(b):
        print(f"SELFTEST FAIL: non-deterministic chunk count {len(a)} != {len(b)}")
        ok = False
    else:
        for ca, cb in zip(a, b):
            if (ca.index != cb.index or ca.lines != cb.lines
                    or ca.inputs != cb.inputs or ca.outputs != cb.outputs):
                print(f"SELFTEST FAIL: non-deterministic chunk {ca.index}")
                ok = False
                break

    # Chunk ids are strict 0-based sequence indices.
    for idx, c in enumerate(a):
        if c.index != idx:
            print(f"SELFTEST FAIL: chunk id {c.index} != sequence index {idx}")
            ok = False

    # Size cap: every chunk's estimate is within the cap, except a chunk whose
    # size is forced past the cap by the bool-no-cross constraint.
    for c in a:
        bool_line_idx = None
        for off, ln in enumerate(c.lines):
            pa = _parse_assign(ln)
            if pa and _is_bool_rhs(pa[1]):
                bool_line_idx = off
        if c.est_lines > 120 and bool_line_idx is None:
            print(f"SELFTEST FAIL: chunk {c.index} est {c.est_lines} > cap 120 "
                  f"with no bool-no-cross justification")
            ok = False

    # The bool var t320 and its use must land in the same chunk (no crossing).
    bool_chunk = None
    use_chunk = None
    for c in a:
        for ln in c.lines:
            if ln.startswith("t320 ="):
                bool_chunk = c.index
            if ln.startswith("t321 ="):
                use_chunk = c.index
    if bool_chunk is None or bool_chunk != use_chunk:
        print(f"SELFTEST FAIL: bool t320 (chunk {bool_chunk}) crossed boundary "
              f"to its use t321 (chunk {use_chunk})")
        ok = False

    # A single small input must come back as exactly one chunk.
    tiny = ["t1 = rho0 + rho1;", "t2 = t1 * 2.0;"]
    _vo2, vd2 = build_dependency_graph(tiny)
    one = partition_compute_lines(tiny, vd2, est, chunk_max_lines=4500)
    if len(one) != 1:
        print(f"SELFTEST FAIL: tiny input produced {len(one)} chunks, expected 1")
        ok = False

    if ok:
        print(f"SELFTEST PASS: {len(a)} chunks, deterministic, cap respected, "
              f"bool-no-cross held")
        return 0
    return 1


if __name__ == '__main__':
    import sys
    if '--selftest' in sys.argv:
        sys.exit(_selftest())
    print("usage: python3 tools/translate_v2/cse.py --selftest")
    sys.exit(2)
