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
# All f64 literals in F arithmetic → F::cast_from(<lit>_f64)
# ---------------------------------------------------------------------------

def test_short_literal_wraps_as_cast_from():
    """`0.5 * t1` → `F::cast_from(0.5_f64) * t1`.

    The former Rule-2 `F::new(0.5)` form narrowed the literal to f32 (CubeCL
    `Float::new(val: f32)`) and tripped rustc's f32-fallback lint #154024.
    All literals now use the exact-f64 `cast_from(<lit>_f64)` form.
    """
    out = per_functional._wrap_f64_literals_v2("0.5 * t1", _let_rhs_F, {})
    assert out == "F::cast_from(0.5_f64) * t1"


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


def test_upstream_f64_turbofish_retargets_to_F_for_allowlist_member():
    """Phase 11.1-01-fix1: family translators (translate_line) emit ::<f64>
    turbofish for ALL paths (flat emission needs ::<f64>; chunked needs ::<F>).

    The chunked v2 pass must retarget `name::<f64>(` -> `name::<F>(` for
    every allowlist member. Without this, chunk bodies inherit the f64
    turbofish (correct for the flat path) and fail to compile in chunked
    form at any F != f64.
    """
    out = per_functional._wrap_f64_literals_v2(
        "pow_1_3::<f64>(t1)", _let_rhs_F, {}
    )
    assert out == "pow_1_3::<F>(t1)"


def test_upstream_f64_turbofish_left_alone_for_non_allowlist():
    """A `::<f64>` turbofish on a non-allowlist name stays untouched (e.g.
    `f64::powf::<f64>(x, y)` is invalid Rust anyway, but the sentinel here
    is `unknown_helper::<f64>(t1)` — leave it for the user to fix manually)."""
    out = per_functional._wrap_f64_literals_v2(
        "unknown_helper::<f64>(t1)", _let_rhs_F, {}
    )
    assert out == "unknown_helper::<f64>(t1)"


def test_f64_method_call_retargets_to_F_in_chunked_body():
    """Phase 11.1-01-fix3: family translator emits `f64::sqrt(x)` for ALL
    paths (correct for flat path). Chunked path retargets to `F::sqrt(x)`
    via the Float-trait associated method form.
    """
    out = per_functional._wrap_f64_literals_v2(
        "f64::sqrt(t1)", _let_rhs_F, {}
    )
    assert out == "F::sqrt(t1)"


def test_f64_compound_call_retargets():
    """`f64::sqrt(f64::EPSILON)` -> `F::sqrt(F::EPSILON)` — both legs swap."""
    out = per_functional._wrap_f64_literals_v2(
        "f64::sqrt(f64::EPSILON)", _let_rhs_F, {}
    )
    assert out == "F::sqrt(F::EPSILON)"


def test_f64_powf_retargets():
    """`f64::powf(t1, t2)` -> `F::powf(t1, t2)`.

    Uses bare-ident args (no nested literals) — the literal-wrap pass would
    otherwise re-wrap a literal that's already inside `F::new(...)` in the
    input. Real chunked bodies arrive bare from translate_line, so the
    re-wrap edge case doesn't bite in production.
    """
    out = per_functional._wrap_f64_literals_v2(
        "f64::powf(t1, t2)", _let_rhs_F, {}
    )
    assert out == "F::powf(t1, t2)"


def test_integer_mantissa_exponent_literal_wraps():
    """Phase 11.1-01-fix4: `1e-21` (integer mantissa + exponent) is a valid
    f64 literal in Rust but the original `_F64_LITERAL_RE` required a
    decimal point. Maple-translated chunks include this shape; without the
    wrap, the chunked body sees `1e-21 * t_F` and fails E0277.
    """
    out = per_functional._wrap_f64_literals_v2(
        "1e-21 * t1", _let_rhs_F, {}
    )
    assert out == "F::cast_from(1e-21_f64) * t1"


def test_integer_mantissa_positive_exponent_wraps():
    """`2e5 * t1` -> `F::cast_from(2e5_f64) * t1`."""
    out = per_functional._wrap_f64_literals_v2(
        "2e5 * t1", _let_rhs_F, {}
    )
    assert out == "F::cast_from(2e5_f64) * t1"


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

def test_qualified_f64_call_retargets_to_F_in_chunked_body():
    """Phase 11.1-01-fix3: chunked bodies retarget `f64::ln(t1)` to `F::ln(t1)`.

    Family translators emit `f64::ln(` for ALL paths (correct for flat fn;
    incorrect for chunked <F: Float> body). The chunked emit's
    _F64_QUALIFIED_RE pass swaps `f64::IDENT` to `F::IDENT` so the Float-
    trait associated method form is used.

    Before fix3 the test asserted the call was left untouched — that
    assumption was incorrect for the chunked path (which is what this v2
    wrap exists to serve).
    """
    out = per_functional._wrap_f64_literals_v2("f64::ln(t1)", _let_rhs_F, {})
    assert out == "F::ln(t1)"


# ---------------------------------------------------------------------------
# All literals (short and long) → F::cast_from(<lit>_f64), exact f64 bit pattern
# ---------------------------------------------------------------------------

def test_all_literals_use_cast_from_with_f64_suffix():
    """`0.5 * 1.7724538509055160` → both as F::cast_from(<lit>_f64)."""
    out = per_functional._wrap_f64_literals_v2(
        "0.5 * 1.7724538509055160", _let_rhs_F, {}
    )
    assert out == "F::cast_from(0.5_f64) * F::cast_from(1.7724538509055160_f64)"


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
    assert out == "F::cast_from(0.5_f64) * t1"


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
    assert rewritten == ["pi * F::cast_from(2.0_f64)", "pi * t1"]
    assert all("M_PI" not in r for r in rewritten)
