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
import enum
import re
from typing import Optional

# --- Splitter v2 knobs (RESEARCH "CSE Detection Heuristic") -------------------
CHUNK_MAX_LINES = 4500          # leave headroom vs the 5000 hard cap (D-LOCK-B)
MIN_REVERSE_DEPS = 5            # natural-breakpoint reverse-dep threshold
MIN_CHAIN_LENGTH = 50           # min def-use chain span before a breakpoint counts
MAX_TUPLE_ARITY = 12            # CubeCL 0.10 CubeType derive ceiling (tuples <=12)
                                # Verified 2026-05-15 via E0277 against 13/14/15-tuples
                                # in 19 subcrates (11-04 Task 1B halt). Do NOT raise
                                # above 12 without re-validating CubeType implementations.


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


def partition_compute_lines(compute_lines, var_deps, est_line_fn, *,
                            chunk_max_lines=CHUNK_MAX_LINES):
    """Partition ``compute_lines`` into deterministic CSE chunks.

    Pass 1 walks the lines in order and cuts them into contiguous index ranges,
    closing a range when its estimated size hits ``chunk_max_lines``, at a
    natural breakpoint (a var with reverse-dep-count >= MIN_REVERSE_DEPS whose
    def-use chain spans >= MIN_CHAIN_LENGTH lines and extends past the current
    line), or when the input/output tuple arity would reach MAX_TUPLE_ARITY.

    Pass 2 turns each range into a Chunk, **rematerializing bools**: a bool
    temporary (``tN = tM <= thresh`` — a comparison with no ternary) is not an
    ``F`` value and cannot cross a D-02 ``<F: Float>`` tuple. Rather than pin a
    bool's whole def-use span into one chunk (the legacy rule, which produced
    single 13-16K-line chunks for the dense 4th-derivative MGGA kernels), a
    bool's one-line definition is simply COPIED into any later chunk that uses
    it — the comparison is cheap, and its operands (non-bool ``F``/``f64``
    values) cross tuples normally. Duplication is bounded (one line per bool per
    using chunk), so there is no string blow-up — unlike the rejected
    inline-substitution approach, which is exponential for ``tE = tC && tC``
    nesting.

    Determinism (D-LOCK-D): range cuts are sequence-ordered; rematerialized bool
    lines are merged in by original program index; tuple inputs are sorted by
    first-use index, outputs by definition index. A bool var is never a chunk
    input or output.

    Args:
        compute_lines: ordered list of C ``tN = expr;`` strings.
        var_deps: dict var -> set(referenced vars). Recomputed internally from
            ``compute_lines`` (the caller's copy may use a different ref
            convention); kept in the signature for compatibility.
        est_line_fn: callable(list[str]) -> int, the caller's Rust line estimator.
        chunk_max_lines: soft per-chunk cap (default CHUNK_MAX_LINES).

    Returns:
        list[Chunk] in deterministic sequence order. A single chunk is returned
        when the whole input already fits under ``chunk_max_lines``.
    """
    # Parse every line once: (var, expr, refs, is_bool, raw_line). `refs` is the
    # set of `tN` temporaries the RHS references (the build_dependency_graph
    # convention) — ambient ids like rho0 / dens_threshold are NOT `tN` and are
    # threaded separately by per_functional.emit_*.
    parsed = []
    for ln in compute_lines:
        pa = _parse_assign(ln)
        if pa is None:
            parsed.append((None, None, set(), False, ln))
            continue
        var, expr = pa
        refs = _refs_of(expr)
        refs.discard(var)
        parsed.append((var, expr, refs, _is_bool_rhs(expr), ln))

    n = len(parsed)
    if n == 0:
        return []

    bool_vars = {p[0] for p in parsed if p[0] is not None and p[3]}

    # First definition index per var, reverse-dep counts, last-use index — over
    # the FULL program, so a non-bool var consumed only by a downstream bool
    # still has a correct last_use (and so still becomes an output / input).
    def_index = {}
    reverse_deps = {}
    last_use = {}
    for i, (var, _expr, refs, _is_bool, _raw) in enumerate(parsed):
        if var is not None and var not in def_index:
            def_index[var] = i
        for r in refs:
            reverse_deps[r] = reverse_deps.get(r, 0) + 1
            last_use[r] = i

    # --- pass 1: cut into contiguous index ranges -----------------------------
    # Bools no longer pin a range open; they are rematerialized in pass 2.
    ranges = []
    start = 0
    defined_in_chunk = set()
    inputs_seen = set()
    lu_buckets = {}              # last_use index -> count of in-range non-bool vars
    i = 0
    while i < n:
        var, _expr, refs, is_bool, _raw = parsed[i]

        for r in refs:
            if (r not in defined_in_chunk and r not in inputs_seen
                    and r not in bool_vars
                    and r in def_index and def_index[r] < start):
                inputs_seen.add(r)

        if var is not None:
            defined_in_chunk.add(var)
            if not is_bool:
                lu = last_use.get(var, i)
                if lu > i:
                    lu_buckets[lu] = lu_buckets.get(lu, 0) + 1

        outputs_open = sum(c for lu, c in lu_buckets.items() if lu > i)
        inputs_arity = len(inputs_seen)

        cur_lines = [parsed[k][4] for k in range(start, i + 1)]
        est = est_line_fn(cur_lines)
        at_last = (i == n - 1)

        natural = False
        if var is not None and not is_bool:
            rd = reverse_deps.get(var, 0)
            lu = last_use.get(var, i)
            di = def_index.get(var, i)
            if rd >= MIN_REVERSE_DEPS and (lu - di) >= MIN_CHAIN_LENGTH and lu > i:
                natural = True

        arity_forced = (inputs_arity >= MAX_TUPLE_ARITY
                        or outputs_open >= MAX_TUPLE_ARITY)

        if at_last or est >= chunk_max_lines or natural or arity_forced:
            ranges.append((start, i))
            start = i + 1
            defined_in_chunk.clear()
            inputs_seen.clear()
            lu_buckets.clear()
        i += 1

    if start < n:
        ranges.append((start, n - 1))

    # --- pass 2: build chunks, rematerializing cross-boundary bools -----------
    bool_refs = {b: parsed[def_index[b]][2] for b in bool_vars}

    chunks = []
    for idx, (s, e) in enumerate(ranges):
        own_idx = set(range(s, e + 1))

        # Bools referenced anywhere in this range but defined OUTSIDE it must be
        # rematerialized. Expand transitively (a bool may reference earlier bools).
        needed = set()
        frontier = []
        for k in range(s, e + 1):
            for r in parsed[k][2]:
                if r in bool_vars and def_index[r] not in own_idx:
                    frontier.append(r)
        while frontier:
            b = frontier.pop()
            if b in needed:
                continue
            needed.add(b)
            for r in bool_refs.get(b, ()):
                if (r in bool_vars and def_index[r] not in own_idx
                        and r not in needed):
                    frontier.append(r)

        # Merge own lines + rematerialized bool def lines, by original program
        # index — a subset of a correctly-ordered program is still correctly
        # ordered, so every line still follows its in-chunk dependencies.
        merged_idx = sorted(own_idx | {def_index[b] for b in needed})
        lines = [parsed[k][4] for k in merged_idx]
        defined_here = {parsed[k][0] for k in merged_idx
                        if parsed[k][0] is not None}

        # inputs: non-bool `tN` vars referenced in the merged lines, defined
        # before this chunk and not (re)defined within it. Bool vars are never
        # inputs (they are rematerialized); ambient ids are threaded elsewhere.
        inputs_first_use = {}
        for k in merged_idx:
            for r in parsed[k][2]:
                if (r not in defined_here and r not in bool_vars
                        and r in def_index and r not in inputs_first_use):
                    inputs_first_use[r] = k
        # Compound key (first-use index, then var name): two vars first used on
        # the SAME line tie on the index, and a bare stable sort would then fall
        # back to dict-insertion order — which is `parsed[k][2]` set-iteration
        # order, i.e. hash-seed-dependent. The var-name tiebreak makes `inputs`
        # — and therefore the emitted chunk signature — deterministic (D-LOCK-D).
        inputs = sorted(inputs_first_use,
                        key=lambda v: (inputs_first_use[v], v))

        # outputs: non-bool vars defined in this chunk's OWN range with a use
        # after the range end. last_use is over the full program, so a var
        # consumed only by a downstream rematerialized bool is still captured.
        outputs = []
        for k in range(s, e + 1):
            v = parsed[k][0]
            if v is not None and v not in bool_vars and last_use.get(v, k) > e:
                outputs.append(v)

        chunks.append(Chunk(index=idx, lines=lines, inputs=inputs,
                             outputs=outputs, est_lines=est_line_fn(lines)))
    return chunks


# --- Phase 11.1 D-04..D-07, D-15, D-19 position classifier ------------------
#
# These additions support per_functional._wrap_f64_literals_v2 — the chunk-body
# emit pass that decides Rule 2 (F::new) vs Rule 3 (F::cast_from with hoisted
# binding) vs native turbofish (D-08) vs no-wrap (Rule 4 doc-comment / string
# guard). Nothing here mutates partition_compute_lines.

class Classification(enum.Enum):
    """Where a token sits in a chunk body, per CONTEXT.md D-04 P-taxonomy."""
    NamedConstInF = "named_const_in_f"            # P1 / hoist via D-05
    BareLiteralTupleMember = "bare_literal_tuple"  # P2 / wrap per D-06
    LetRhsToF = "let_rhs_to_f"                    # P3 / wrap per D-06
    HelperArg = "helper_arg"                       # P4 / wrap per D-06
    SingleExprBodyReturn = "single_expr_body"      # P5 / wrap per D-06
    MatchOrCondOrClosure = "match_or_cond"        # P6 / wrap per D-06
    IntegerCounter = "integer_counter"             # no-wrap (Rule 4 / index)


@dataclass
class PositionContext:
    """Context passed to ``classify_position`` for D-07 position-driven dispatch.

    The classifier looks at WHERE a token sits — not what it is — to decide
    whether to wrap. ``parent_kind`` names the immediate syntactic position
    (`multiplicand`, `tuple_return_member`, `let_rhs`, `helper_call_arg`,
    `single_expr_body`, `match_arm`, `if_branch`, `closure_body`, `index`,
    `range`, `u32_arg`, `u16_arg`, `usize_arg`). ``enclosing_return_type``
    is the textual return-type form of the enclosing fn (`F`, `(F, F)`,
    `(F, F, F)`, `_`). ``enclosing_let_type`` is `"F"` when known, `None`
    otherwise. The two boolean guards (`in_doc_comment`, `in_string_literal`)
    implement the Rule-4 hands-off boundary.
    """
    parent_kind: str
    enclosing_return_type: str
    enclosing_let_type: Optional[str]
    in_doc_comment: bool
    in_string_literal: bool


# Lazy import — kept inside classify_position to avoid a circular import
# in case helpers_allowlist ever grows a dependency on this module.
def _named_consts() -> dict:
    from translate_v2.helpers_allowlist import NAMED_CONSTS
    return NAMED_CONSTS


_INTEGER_LITERAL_RE = re.compile(r"^\d+$")
_INTEGER_PARENT_KINDS = frozenset(
    {"index", "range", "u32_arg", "u16_arg", "usize_arg"}
)
_TUPLE_F_RETURN_RE = re.compile(r"^\(\s*F\s*,")
_CONDITIONAL_PARENT_KINDS = frozenset(
    {"match_arm", "if_branch", "closure_body"}
)


def classify_position(token: str, ctx: PositionContext) -> Classification:
    """Classify a token by its syntactic position (D-07 decision tree).

    Decision order (per Plan 11.1-01 Task 1):
        1. Doc-comment / string-literal guard → IntegerCounter (no-wrap sentinel)
        2. Pure-integer in a counter-style parent → IntegerCounter
        3. Named-const in F-typed enclosing return → NamedConstInF
        4. tuple-return member in (F, ...) → BareLiteralTupleMember
        5. let-rhs with enclosing F let-type → LetRhsToF
        6. helper-call arg → HelperArg
        7. single-expression body in F-typed fn → SingleExprBodyReturn
        8. match-arm / if-branch / closure-body → MatchOrCondOrClosure
        9. fallback → IntegerCounter (safe no-wrap default)
    """
    # 1. Rule-4 hands-off: doc-comments and string literals are never wrapped.
    if ctx.in_doc_comment or ctx.in_string_literal:
        return Classification.IntegerCounter

    # 2. Pure integer in a counter/index/u32-arg position.
    if _INTEGER_LITERAL_RE.match(token) and ctx.parent_kind in _INTEGER_PARENT_KINDS:
        return Classification.IntegerCounter

    # 3. Named-const in an F-typed enclosing context (P1 / P3 / P4 / P5 with
    # named-const dimension). The named-const dimension is position-orthogonal
    # by D-04 design — it beats the let-rhs / helper-arg / single-expr-body
    # bare-literal classifications when the token is itself a named-const.
    if token in _named_consts():
        ret = ctx.enclosing_return_type
        if ret == "F" or _TUPLE_F_RETURN_RE.match(ret or ""):
            return Classification.NamedConstInF

    # 4. Bare literal as a tuple-return member.
    if (ctx.parent_kind == "tuple_return_member"
            and _TUPLE_F_RETURN_RE.match(ctx.enclosing_return_type or "")):
        return Classification.BareLiteralTupleMember

    # 5. Bare literal on the RHS of a let-binding into an F-typed local.
    if ctx.parent_kind == "let_rhs" and ctx.enclosing_let_type == "F":
        return Classification.LetRhsToF

    # 6. Bare literal as a helper-call argument.
    if ctx.parent_kind == "helper_call_arg":
        return Classification.HelperArg

    # 7. Bare literal as a single-expression body return.
    if (ctx.parent_kind == "single_expr_body"
            and ctx.enclosing_return_type == "F"):
        return Classification.SingleExprBodyReturn

    # 8. Bare literal in match-arm RHS, if-branch RHS, or closure body.
    if ctx.parent_kind in _CONDITIONAL_PARENT_KINDS:
        return Classification.MatchOrCondOrClosure

    # 9. Safe fallback: no-wrap.
    return Classification.IntegerCounter


# Match candidate UPPER_SNAKE_CASE identifiers + the special ``f64::MAX`` form.
# Word-boundary anchors keep us from biting into surrounding tokens; the
# ``f64::MAX`` alternative is matched literally because ``::`` is not a
# word character.
_UPPER_CONST_RE = re.compile(r"\b[A-Z][A-Z0-9_]*\b|\bf64::MAX\b")


def collect_named_const_uses(rust_body: str) -> dict:
    """Frequency map of NAMED_CONSTS occurrences in ``rust_body``.

    Excludes occurrences inside doc-comments (``///`` and ``//!`` prefix
    lines) and inside string-literal spans (per-line even/odd `"` tracking).
    Used by ``per_functional.py`` to build the per-chunk hoisting prelude.

    Returns ``{symbol: count}`` with ``count >= 1``. Symbols not in
    :data:`helpers_allowlist.NAMED_CONSTS` are filtered out.
    """
    named = _named_consts()
    counts: dict = {}
    for line in rust_body.splitlines():
        stripped = line.lstrip()
        # Skip doc-comment lines wholesale.
        if stripped.startswith("///") or stripped.startswith("//!"):
            continue
        # Build a mask of "inside a string-literal" character positions by
        # walking the line and toggling on each (unescaped) `"`. The mask is
        # then consulted at every regex match start.
        in_string = False
        mask = [False] * len(line)
        i = 0
        while i < len(line):
            ch = line[i]
            # Treat `\"` as an escaped quote — does not flip the string state.
            if ch == "\\" and i + 1 < len(line):
                mask[i] = in_string
                mask[i + 1] = in_string
                i += 2
                continue
            if ch == '"':
                # Flip AFTER recording the position so the quote itself sits in
                # whichever side opened it. We treat the quote characters as
                # part of the string span for masking purposes.
                in_string = not in_string
                mask[i] = True
                i += 1
                continue
            mask[i] = in_string
            i += 1
        for m in _UPPER_CONST_RE.finditer(line):
            sym = m.group(0)
            if sym not in named:
                continue
            if mask[m.start()]:
                continue
            counts[sym] = counts.get(sym, 0) + 1
    return counts


# D-05 hoisting-prelude line template. Filled with `name` (the local binding,
# e.g. `pi`) and `symbol` (the source const, e.g. `M_PI`):
#     "    let pi = F::cast_from(M_PI);"
HOIST_PRELUDE_TEMPLATE = "    let {name} = F::cast_from({symbol});"


# --- self-test ---------------------------------------------------------------
def _selftest():
    """Run a synthetic compute_lines list through partition_compute_lines.

    Asserts the partition is deterministic across two calls and that no chunk
    exceeds chunk_max_lines. Exit 0 on success, 1 on failure.
    """
    # Synthetic kernel: a long linear chain with a bool defined EARLY and used
    # FAR downstream — forcing a cross-range bool rematerialization.
    lines = []
    lines.append("t1 = rho0 + rho1;")
    lines.append("t2 = t1 <= 0.1e-14;")           # bool, defined early
    for k in range(3, 320):
        lines.append(f"t{k} = t{k-1} * 0.5 + t1;")
    lines.append("t320 = t2 ? t1 : t319;")        # uses the early bool, far away
    for k in range(321, 640):
        lines.append(f"t{k} = t{k-1} - t320;")

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

    # Size cap: ranges are cut at the cap; rematerialized bools add only a few
    # lines, never the 13-16K blow-up the legacy bool-no-cross rule caused.
    for c in a:
        if c.est_lines > 120 + 25:
            print(f"SELFTEST FAIL: chunk {c.index} est {c.est_lines} far over cap 120")
            ok = False

    # Bool rematerialization: t2 is defined early and used far downstream, so
    # its one-line definition must be COPIED into the using chunk — appearing
    # in >=2 chunks total — and a bool var must never become a tuple input.
    t2_chunks = [c.index for c in a
                 if any(ln.lstrip().startswith("t2 =") for ln in c.lines)]
    if len(t2_chunks) < 2:
        print(f"SELFTEST FAIL: bool t2 not rematerialized (found in chunks {t2_chunks})")
        ok = False
    if any("t2" in c.inputs for c in a):
        print("SELFTEST FAIL: bool var t2 leaked into a chunk's tuple inputs")
        ok = False
    for c in a:
        uses_t2 = any(ln.lstrip().startswith("t320 =") for ln in c.lines)
        has_t2_def = any(ln.lstrip().startswith("t2 =") for ln in c.lines)
        if uses_t2 and not has_t2_def:
            print(f"SELFTEST FAIL: chunk {c.index} uses t2 but lacks its rematerialized def")
            ok = False

    # A single small input must come back as exactly one chunk.
    tiny = ["t1 = rho0 + rho1;", "t2 = t1 * 2.0;"]
    _vo2, vd2 = build_dependency_graph(tiny)
    one = partition_compute_lines(tiny, vd2, est, chunk_max_lines=4500)
    if len(one) != 1:
        print(f"SELFTEST FAIL: tiny input produced {len(one)} chunks, expected 1")
        ok = False

    if ok:
        print(f"SELFTEST PASS: {len(a)} chunks, deterministic, cap held, "
              f"bools rematerialized")
        return 0
    return 1


if __name__ == '__main__':
    import sys
    if '--selftest' in sys.argv:
        sys.exit(_selftest())
    print("usage: python3 tools/translate_v2/cse.py --selftest")
    sys.exit(2)
