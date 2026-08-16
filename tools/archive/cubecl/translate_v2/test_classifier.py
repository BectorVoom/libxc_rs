#!/usr/bin/env python3
"""Tests for translate_v2.cse position classifier + named-const collector
and the translate_v2.helpers_allowlist module surface.

These tests pin the D-04..D-07 position-driven classification semantics and
the D-09 hardcoded helper allowlist shape. They are pure-Python unit tests
(no Rust compile, no cargo) — fast feedback for the translator amendment
in Phase 11.1 Plan 01 Task 1.

Run via: `python3 -m pytest tools/translate_v2/test_classifier.py -q`
"""

import re
import sys
from pathlib import Path

# Allow `from translate_v2 import ...` when run from the repo root.
sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from translate_v2.cse import (  # noqa: E402  -- sys.path mutation above
    PositionContext,
    Classification,
    classify_position,
    collect_named_const_uses,
    HOIST_PRELUDE_TEMPLATE,
)
from translate_v2.helpers_allowlist import (  # noqa: E402
    GENERIC_HELPERS,
    NON_GENERIC_MODULES,
    NAMED_CONSTS,
    is_generic_helper_call,
)


# ---------------------------------------------------------------------------
# classify_position decision-tree tests (P1..P6 + Rule-4 guard + counter)
# ---------------------------------------------------------------------------

def test_p1_named_const_in_f_arithmetic():
    """P1: M_PI used as a multiplicand in an F-typed expression."""
    ctx = PositionContext(
        parent_kind="multiplicand",
        enclosing_return_type="F",
        enclosing_let_type=None,
        in_doc_comment=False,
        in_string_literal=False,
    )
    assert classify_position("M_PI", ctx) == Classification.NamedConstInF


def test_p2_bare_literal_tuple_return_member():
    """P2: 0.5 as a member of (F, F, F) tuple-return."""
    ctx = PositionContext(
        parent_kind="tuple_return_member",
        enclosing_return_type="(F, F, F)",
        enclosing_let_type=None,
        in_doc_comment=False,
        in_string_literal=False,
    )
    assert classify_position("0.5", ctx) == Classification.BareLiteralTupleMember


def test_p3_named_const_in_let_rhs_beats_let_rhs_classification():
    """P3: named const in let-rhs is still classified as NamedConstInF.

    The named-const dimension beats the bare-let-rhs dimension by design —
    let-rhs classification is for BARE LITERALS only (D-04 / D-07).
    """
    ctx = PositionContext(
        parent_kind="let_rhs",
        enclosing_return_type="F",
        enclosing_let_type="F",
        in_doc_comment=False,
        in_string_literal=False,
    )
    assert classify_position("M_CBRT3", ctx) == Classification.NamedConstInF


def test_p4_named_const_in_helper_arg_beats_helper_arg_classification():
    """P4: named const in helper-call arg is still NamedConstInF."""
    ctx = PositionContext(
        parent_kind="helper_call_arg",
        enclosing_return_type="F",
        enclosing_let_type=None,
        in_doc_comment=False,
        in_string_literal=False,
    )
    assert classify_position("M_PI", ctx) == Classification.NamedConstInF


def test_p5_named_const_in_single_expr_body():
    """P5: named const as single-expression body return."""
    ctx = PositionContext(
        parent_kind="single_expr_body",
        enclosing_return_type="F",
        enclosing_let_type=None,
        in_doc_comment=False,
        in_string_literal=False,
    )
    assert classify_position("M_CBRT3", ctx) == Classification.NamedConstInF


def test_p6_bare_literal_in_match_arm():
    """P6: bare 0.0 in a match-arm body."""
    ctx = PositionContext(
        parent_kind="match_arm",
        enclosing_return_type="F",
        enclosing_let_type=None,
        in_doc_comment=False,
        in_string_literal=False,
    )
    assert classify_position("0.0", ctx) == Classification.MatchOrCondOrClosure


def test_integer_counter_in_index_position():
    """Bare integer in indexing position: never wrap."""
    ctx = PositionContext(
        parent_kind="index",
        enclosing_return_type="_",
        enclosing_let_type=None,
        in_doc_comment=False,
        in_string_literal=False,
    )
    assert classify_position("3", ctx) == Classification.IntegerCounter


def test_rule4_doc_comment_guard_returns_counter_sentinel():
    """Rule 4: anything inside a doc-comment is no-wrap (IntegerCounter sentinel)."""
    ctx = PositionContext(
        parent_kind="multiplicand",
        enclosing_return_type="F",
        enclosing_let_type=None,
        in_doc_comment=True,
        in_string_literal=False,
    )
    assert classify_position("M_PI", ctx) == Classification.IntegerCounter


# ---------------------------------------------------------------------------
# collect_named_const_uses tests (doc/string handling)
# ---------------------------------------------------------------------------

def test_collect_named_const_uses_p1_exemplar():
    """The exact P1 exemplar from .continue-here.md (gga_c_gaploc chunk804.rs:13)."""
    body = "let t40620 = t9105 * t5337 * M_PI * t1691 * SQRT_DBL_EPSILON;\n"
    assert collect_named_const_uses(body) == {"M_PI": 1, "SQRT_DBL_EPSILON": 1}


def test_collect_named_const_uses_excludes_doc_comment_line():
    """Doc-comment lines (///) are skipped; only code lines count.

    Per the plan: collect_named_const_uses iterates line-by-line and skips
    lines whose lstrip starts with `///` or `//!`. The code-line M_PI counts.
    """
    body = "/// pi = M_PI = 3.14...\nlet x = M_PI;\n"
    assert collect_named_const_uses(body) == {"M_PI": 1}


def test_collect_named_const_uses_excludes_string_literal_span():
    """M_PI inside "..." is excluded; M_PI outside the string is counted."""
    body = 'let s = "M_PI"; let x = M_PI;\n'
    assert collect_named_const_uses(body) == {"M_PI": 1}


# ---------------------------------------------------------------------------
# helpers_allowlist shape tests
# ---------------------------------------------------------------------------

def test_generic_helpers_has_exactly_14_modules():
    """D-09 mandates 14 generic-helper modules."""
    assert len(GENERIC_HELPERS) == 14, (
        f"GENERIC_HELPERS has {len(GENERIC_HELPERS)} entries, expected 14; "
        f"keys={sorted(GENERIC_HELPERS)}"
    )


def test_generic_helpers_disjoint_from_non_generic_modules():
    """A module cannot be both generic and non-generic."""
    overlap = set(GENERIC_HELPERS) & set(NON_GENERIC_MODULES)
    assert overlap == set(), (
        f"GENERIC_HELPERS ∩ NON_GENERIC_MODULES must be empty; got {overlap}"
    )


def test_non_generic_modules_is_exactly_constants_deferred_lib():
    """D-09 non-generic exclusions: constants, deferred, lib."""
    assert NON_GENERIC_MODULES == frozenset({"deferred", "constants", "lib"})


def test_named_consts_has_exactly_22_entries():
    """D-19 named-const name map has 22 entries."""
    assert len(NAMED_CONSTS) == 22, (
        f"NAMED_CONSTS has {len(NAMED_CONSTS)} entries, expected 22; "
        f"keys={list(NAMED_CONSTS)}"
    )


def test_named_consts_values_are_valid_rust_identifiers():
    """Every hoisted-binding name must be a valid lowercase snake_case Rust ident."""
    ident_re = re.compile(r"^[a-z][a-z0-9_]*$")
    bad = {k: v for k, v in NAMED_CONSTS.items() if not ident_re.match(v)}
    assert bad == {}, f"Invalid Rust idents in NAMED_CONSTS: {bad}"


def test_is_generic_helper_call_smoke():
    """A small smoke test for the membership helper across modules."""
    assert is_generic_helper_call("pow_1_3") is True            # powers
    assert is_generic_helper_call("piecewise5") is True         # piecewise
    assert is_generic_helper_call("xc_mgga_x_br89_get_x") is True  # br89
    assert is_generic_helper_call("xc_bessel_I0_scaled") is True   # bessel
    assert is_generic_helper_call("is_deferred") is False       # non-generic
    assert is_generic_helper_call("totally_unknown_helper") is False


def test_hoist_prelude_template_format():
    """The template must expand `name` and `symbol` placeholders."""
    line = HOIST_PRELUDE_TEMPLATE.format(name="pi", symbol="M_PI")
    assert line == "    let pi = F::cast_from(M_PI);"
