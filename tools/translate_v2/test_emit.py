#!/usr/bin/env python3
"""Tests for translate_v2.per_functional._wrap_f64_literals_v2 — the chunk-body
emit pass that combines D-04..D-08, D-10, and D-19 logic in one function.

Run via: `python3 -m pytest tools/translate_v2/test_emit.py -q`
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from translate_v2.cse import PositionContext  # noqa: E402
from translate_v2 import per_functional  # noqa: E402

_let_rhs_F = PositionContext(
    parent_kind="let_rhs",
    enclosing_return_type="F",
    enclosing_let_type="F",
    in_doc_comment=False,
    in_string_literal=False,
)


# ---------------------------------------------------------------------------
# Rule 2: short f64 literal in F arithmetic → F::new(<lit>)
# ---------------------------------------------------------------------------

def test_short_literal_wraps_as_f_new():
    """`0.5 * t1` → `F::new(0.5) * t1` (D-06 Rule 2)."""
    out = per_functional._wrap_f64_literals_v2("0.5 * t1", _let_rhs_F, {})
    assert out == "F::new(0.5) * t1"


# ---------------------------------------------------------------------------
# D-05 hoisted prelude substitution
# ---------------------------------------------------------------------------

def test_hoisted_named_const_substitutes_to_local_binding():
    """`M_PI * t1` with {M_PI: pi} hoisted → `pi * t1`."""
    out = per_functional._wrap_f64_literals_v2(
        "M_PI * t1", _let_rhs_F, {"M_PI": "pi"}
    )
    assert out == "pi * t1"


# ---------------------------------------------------------------------------
# D-08 native helper turbofish via D-09 allowlist
# ---------------------------------------------------------------------------

def test_allowlist_helper_call_gets_native_turbofish():
    """`pow_1_3(t1)` → `pow_1_3::<F>(t1)` (D-08 / D-09 allowlist member)."""
    out = per_functional._wrap_f64_literals_v2("pow_1_3(t1)", _let_rhs_F, {})
    assert out == "pow_1_3::<F>(t1)"


def test_unknown_helper_call_left_untouched():
    """Calls not in GENERIC_HELPERS allowlist stay as-is (e.g. `is_deferred`,
    `unknown_helper`)."""
    out = per_functional._wrap_f64_literals_v2(
        "unknown_helper(t1)", _let_rhs_F, {}
    )
    assert out == "unknown_helper(t1)"


# ---------------------------------------------------------------------------
# D-10 chunk-to-chunk turbofish (inside a generic chunk body)
# ---------------------------------------------------------------------------

def test_chunk_to_chunk_call_gets_F_turbofish():
    """A chunk fn calling another chunk fn from inside its body emits ::<F>."""
    out = per_functional._wrap_f64_literals_v2(
        "foo_chunk5(t1, t2)", _let_rhs_F, {}
    )
    assert out == "foo_chunk5::<F>(t1, t2)"


# ---------------------------------------------------------------------------
# f64:: qualified calls are left untouched
# ---------------------------------------------------------------------------

def test_qualified_f64_call_left_untouched():
    """`f64::ln(t1)` is a concrete-typed call — never gets turbofish."""
    out = per_functional._wrap_f64_literals_v2("f64::ln(t1)", _let_rhs_F, {})
    assert out == "f64::ln(t1)"


# ---------------------------------------------------------------------------
# D-06 long-literal precision split: > 8 significant digits → F::cast_from(_f64)
# ---------------------------------------------------------------------------

def test_long_literal_uses_cast_from_with_f64_suffix():
    """`0.5 * 1.7724538509055160` → mixed: F::new(short) and F::cast_from(long_f64)."""
    out = per_functional._wrap_f64_literals_v2(
        "0.5 * 1.7724538509055160", _let_rhs_F, {}
    )
    assert out == "F::new(0.5) * F::cast_from(1.7724538509055160_f64)"


# ---------------------------------------------------------------------------
# Rule-4 doc-comment guard — nothing gets wrapped
# ---------------------------------------------------------------------------

def test_doc_comment_context_leaves_expression_untouched():
    """When ctx.in_doc_comment is True the input is returned verbatim."""
    ctx = PositionContext(
        parent_kind="multiplicand",
        enclosing_return_type="F",
        enclosing_let_type=None,
        in_doc_comment=True,
        in_string_literal=False,
    )
    out = per_functional._wrap_f64_literals_v2("M_PI * t1", ctx, {})
    assert out == "M_PI * t1"


# ---------------------------------------------------------------------------
# Backwards-compat shim: legacy _wrap_f64_literals still exists and wraps
# short literals (covers any external smoke test that imports the old name).
# ---------------------------------------------------------------------------

def test_backwards_compat_shim_wraps_short_literal():
    out = per_functional._wrap_f64_literals("0.5 * t1")
    assert out == "F::new(0.5) * t1"


# ---------------------------------------------------------------------------
# Integration: hoisting prelude in a tiny synthetic chunk emit
# ---------------------------------------------------------------------------

def test_prelude_emitted_once_per_referenced_named_const():
    """Pre-scan + prelude emission: a body that uses M_PI twice gets exactly
    one prelude line and zero unsubstituted `M_PI *` occurrences."""
    from translate_v2.cse import collect_named_const_uses, HOIST_PRELUDE_TEMPLATE
    from translate_v2.helpers_allowlist import NAMED_CONSTS

    body_preview = (
        "let t1 = M_PI * 2.0;\n"
        "let t2 = M_PI * t1;\n"
    )
    uses = collect_named_const_uses(body_preview)
    assert uses == {"M_PI": 2}

    hoisted = {sym: NAMED_CONSTS[sym] for sym in uses if sym in NAMED_CONSTS}
    assert hoisted == {"M_PI": "pi"}

    prelude_lines = [
        HOIST_PRELUDE_TEMPLATE.format(name=hoisted[sym], symbol=sym)
        for sym in hoisted
    ]
    assert prelude_lines == ["    let pi = F::cast_from(M_PI);"]

    # Substitute each chunk line through _wrap_f64_literals_v2 with the
    # hoisted map — both M_PI references should turn into the local `pi`.
    rewritten = [
        per_functional._wrap_f64_literals_v2(rhs, _let_rhs_F, hoisted)
        for rhs in ["M_PI * 2.0", "M_PI * t1"]
    ]
    assert rewritten == ["pi * F::new(2.0)", "pi * t1"]
    assert all("M_PI" not in r for r in rewritten)
