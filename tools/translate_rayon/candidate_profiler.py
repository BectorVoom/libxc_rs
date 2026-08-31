#!/usr/bin/env python3
"""Candidate profiler and eligibility reporter for SIMD math kernel optimization.

Analyzes all maple2c functional sources across (order, spin) triples to determine:
1. Counts of elementary math calls (exp, ln, cbrt, atan, tanh, etc.)
2. Presence of scalar helpers (lambert_w, xc_e1_scaled, br89, mbrxc, bessel, etc.)
3. Estimated transcendental cost
4. Routing / support status in libxc-reval
5. Exact SIMD eligibility and priority tier assignment (P0, P1, P2, P3, P4)
"""
from __future__ import annotations

import collections
import json
import re
from pathlib import Path
import from_maple

REPO = Path(__file__).resolve().parents[2]
MAPLE = REPO / "libxc-master" / "src" / "maple2c"
ROUTING_RS = REPO / "crates" / "libxc-reval" / "src" / "routing.rs"

# Microbenchmark cost per call in nanoseconds (from baseline benchmarks)
COST_TABLE = {
    "exp": 2.27,
    "ln": 1.96,
    "cbrt": 5.85,
    "atan": 8.10,
    "tanh": 10.55,
    "sinh": 10.0,
    "cosh": 10.0,
    "atanh": 8.0,
    "sin": 5.0,
    "cos": 5.0,
    "tan": 8.0,
    "erf": 6.17,
    "erfc": 6.17,
    "pow": 8.50,
    "asin": 8.0,
    "acos": 8.0,
    "expm1": 2.50,
    "log1p": 2.20,
    "atan2": 9.0,
}

SCALAR_HELPERS = {
    "lambert_w": "lambert_w",
    "LambertW": "lambert_w",
    "xc_E1_scaled": "xc_e1_scaled",
    "xc_dilogarithm": "xc_dilogarithm",
    "xc_erfcx": "xc_erfcx",
    "xc_mgga_x_br89_get_x": "xc_mgga_x_br89_get_x",
    "xc_mgga_x_mbrxc_get_x": "xc_mgga_x_mbrxc_get_x",
    "xc_bessel_I0": "bessel",
    "xc_bessel_I1": "bessel",
    "xc_bessel_K0": "bessel",
    "xc_bessel_K1": "bessel",
    "xc_bessel_I0_scaled": "bessel",
    "xc_bessel_I1_scaled": "bessel",
    "xc_bessel_K0_scaled": "bessel",
    "xc_bessel_K1_scaled": "bessel",
    "xc_integrate": "integrate",
    "xbspline": "bspline",
    "cbspline": "bspline",
}

CALL_NAME_MAP = {
    "exp": "exp",
    "log": "ln",
    "cbrt": "cbrt",
    "POW_1_3": "cbrt",
    "atan": "atan",
    "tanh": "tanh",
    "sinh": "sinh",
    "cosh": "cosh",
    "asinh": "asinh",
    "acosh": "acosh",
    "atanh": "atanh",
    "sin": "sin",
    "cos": "cos",
    "tan": "tan",
    "erf": "erf",
    "erfc": "erfc",
    "pow": "pow",
    "asin": "asin",
    "acos": "acos",
    "expm1": "expm1",
    "log1p": "log1p",
    "atan2": "atan2",
}

ELEM_RE = re.compile(r"\b(" + "|".join(CALL_NAME_MAP.keys()) + r")\s*\(")
HELPER_RE = re.compile(r"\b(" + "|".join(SCALAR_HELPERS.keys()) + r")\b")

EXACT_SUPPORTED_TRANSCENDENTALS = {
    "exp", "ln", "cbrt", "pow_1_3", "pow_2_3", "pow_4_3", "pow_5_3", "pow_7_3",
    "expm1", "log1p", "atan", "atan2", "tanh", "sinh", "cosh", "atanh",
    "sin", "cos", "tan", "erf", "erfc", "pow", "asin", "acos", "sqrt", "abs"
}


def load_unsupported() -> dict[str, str]:
    unsupported = {}
    if ROUTING_RS.is_file():
        text = ROUTING_RS.read_text()
        for m in re.finditer(r'\("([^"]+)",\s*"([^"]+)"\)', text):
            unsupported[m.group(1)] = m.group(2)
    return unsupported


def analyze_function_body(body: str) -> tuple[dict[str, int], set[str], int]:
    """Return (math_call_counts, helper_set, statement_count)."""
    counts: dict[str, int] = collections.defaultdict(int)
    helpers: set[str] = set()
    
    stmt_count = body.count(";")
    
    for m in ELEM_RE.finditer(body):
        raw_name = m.group(1)
        canonical = CALL_NAME_MAP[raw_name]
        counts[canonical] += 1
            
    for m in HELPER_RE.finditer(body):
        h_name = SCALAR_HELPERS[m.group(1)]
        helpers.add(h_name)
            
    return dict(counts), helpers, stmt_count


def profile_all():
    unsupported = load_unsupported()
    files = from_maple.maple_files()
    
    records = []
    
    for func, path in sorted(files.items()):
        text = path.read_text(errors="ignore")
        fam = from_maple.family_of(path)
        is_routed = func not in unsupported
        unsupported_reason = unsupported.get(func, "")
        
        fns = from_maple.split_functions(text)
        for (order, spin), body in fns.items():
            counts, helpers, stmt_count = analyze_function_body(body)
            tot_calls = sum(counts.values())
            est_cost = sum(counts.get(k, 0) * COST_TABLE.get(k, 2.0) for k in counts)
            
            # Check if all elementary calls are exact-translatable
            all_exact = all(k in EXACT_SUPPORTED_TRANSCENDENTALS for k in counts)
            
            # Queue assignment
            queue = "P_UNROUTED"
            if is_routed:
                if (func, order, spin) in from_maple.SIMD_EXACT_FUNCS:
                    queue = "P0"
                elif spin == "unpol" and order in ("exc", "vxc"):
                    if helpers:
                        if "lambert_w" in helpers or "xc_e1_scaled" in helpers or "special" in helpers or "bessel" in helpers:
                            queue = "P2"
                        else:
                            queue = "P4"
                    elif tot_calls >= 2 and all_exact:
                        queue = "P1"
                    else:
                        queue = "P_LOW_BENEFIT"
                elif spin == "pol" or order in ("fxc", "kxc", "lxc"):
                    queue = "P3"
                else:
                    queue = "P_OTHER"
            
            records.append({
                "func": func,
                "fam": fam,
                "order": order,
                "spin": spin,
                "routed": is_routed,
                "unsupported_reason": unsupported_reason,
                "calls": counts,
                "tot_calls": tot_calls,
                "helpers": sorted(helpers),
                "stmt_count": stmt_count,
                "est_cost_ns": est_cost,
                "all_exact": all_exact,
                "queue": queue,
            })
            
    return records


def main():
    records = profile_all()
    
    print("================================================================================")
    print("  SIMD Math-Kernel Eligibility & Candidate Priority Report")
    print("================================================================================")
    
    p0 = [r for r in records if r["queue"] == "P0"]
    p1 = [r for r in records if r["queue"] == "P1"]
    p2 = [r for r in records if r["queue"] == "P2"]
    p3 = [r for r in records if r["queue"] == "P3" and r["tot_calls"] >= 2]
    
    print(f"\n[P0] Current Promoted Exact SIMD Triples: {len(p0)} triples")
    print(f"{'Functional':<25} {'Order':<6} {'Spin':<6} {'Calls':<6} {'Est ns':<8} {'Stmts':<6} {'Breakdown'}")
    print("-" * 80)
    for r in sorted(p0, key=lambda x: -x["est_cost_ns"]):
        calls_str = ", ".join(f"{k}:{v}" for k, v in sorted(r["calls"].items()))
        print(f"{r['func']:<25} {r['order']:<6} {r['spin']:<6} {r['tot_calls']:<6} {r['est_cost_ns']:<8.1f} {r['stmt_count']:<6} {calls_str}")

    print(f"\n[P1] Routed Unpolarized (exc/vxc) Exact Candidates (no helpers, >=2 libm calls): {len(p1)} triples")
    print(f"{'Functional':<25} {'Order':<6} {'Spin':<6} {'Calls':<6} {'Est ns':<8} {'Stmts':<6} {'Breakdown'}")
    print("-" * 80)
    for r in sorted(p1, key=lambda x: -x["est_cost_ns"]):
        calls_str = ", ".join(f"{k}:{v}" for k, v in sorted(r["calls"].items()))
        print(f"{r['func']:<25} {r['order']:<6} {r['spin']:<6} {r['tot_calls']:<6} {r['est_cost_ns']:<8.1f} {r['stmt_count']:<6} {calls_str}")
        
    print(f"\n[P2] Candidates Requiring Vector Helper Functions: {len(p2)} triples")
    print(f"{'Functional':<25} {'Order':<6} {'Spin':<6} {'Helpers':<25} {'Calls':<6} {'Est ns':<8}")
    print("-" * 80)
    for r in sorted(p2, key=lambda x: -x["est_cost_ns"])[:15]:
        helpers_str = ", ".join(r["helpers"])
        print(f"{r['func']:<25} {r['order']:<6} {r['spin']:<6} {helpers_str:<25} {r['tot_calls']:<6} {r['est_cost_ns']:<8.1f}")


if __name__ == "__main__":
    main()
