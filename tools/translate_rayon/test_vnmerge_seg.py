#!/usr/bin/env python3
"""Unit tests for the segmented emission path in vnmerge.py.

These cover the plan (cut placement, slot reuse) and the shape of the emitted
text. Numerical equivalence is not a unit-test property here: it is established
by the old-vs-new bitwise harness over a real grid, which is the only check
that can see what rustc and libm actually do.

Run: python3 tools/translate_rayon/test_vnmerge_seg.py
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from vnmerge import (  # noqa: E402
    MergeError, emit_segmented, merge_texts, plan_segments, _alloc_slots,
    _analyze, _is_bool_rhs, _live_after, _value_types,
)

FAILED: list[str] = []


def check(cond: bool, what: str) -> None:
    if cond:
        print(f"  ok   {what}")
    else:
        print(f"  FAIL {what}")
        FAILED.append(what)


def chain(n: int, *, out_name: str = "zk", stride: int = 1) -> list[str]:
    """A merged statement list: a linear chain of defs then one store."""
    stmts = ["        let v0 = rho[ip * 2];"]
    for i in range(1, n):
        stmts.append(f"        let v{i} = v{i - 1} * 1.5;")
    idx = "ip" if stride == 1 else f"ip * {stride}"
    stmts.append(f"        {out_name}[{idx}] += v{n - 1};")
    return stmts


def test_analyze() -> None:
    print("test_analyze")
    out = [
        "        let v0 = rho[ip * 2];",
        "        let v1 = v0 * v0;",
        "        v2rho2[ip * 3 + 1] += v1;",
    ]
    def_pos, last_use, uses = _analyze(out)
    check(def_pos == {0: 0, 1: 1}, "def positions found")
    check(uses[1] == {0}, "use set of a binary op")
    # `v2rho2` must not be read as value id 2: it fails the trailing boundary.
    check(uses[2] == {1}, "output buffer name is not mistaken for a value id")
    check(last_use == {0: 1, 1: 2}, "last uses tracked through the store")


def test_live_width() -> None:
    print("test_live_width")
    out = chain(6)
    def_pos, last_use, _ = _analyze(out)
    width = _live_after(out, def_pos, last_use)
    check(all(w <= 1 for w in width), "a linear chain is never more than 1 live")
    # A value used much later keeps the width up in between.
    out2 = [
        "        let v0 = rho[ip];",
        "        let v1 = v0 * 2.0;",
        "        let v2 = v1 * 3.0;",
        "        let v3 = v2 + v0;",
        "        zk[ip] += v3;",
    ]
    def_pos, last_use, _ = _analyze(out2)
    w = _live_after(out2, def_pos, last_use)
    check(w[1] == 2, "a long-lived value raises the live count")


def test_slot_reuse() -> None:
    print("test_slot_reuse")
    # Two values with disjoint segment intervals must share one slot.
    def_pos = {0: 0, 1: 10}
    last_use = {0: 5, 1: 15}
    seg_of = [0] * 6 + [1] * 5 + [2] * 5 + [3]
    slot_of, n = _alloc_slots(def_pos, last_use, seg_of)
    # v0 lives entirely inside segment 0, so it never crosses and needs no slot.
    check(slot_of == {1: 0} and n == 1, "a value that never crosses gets no slot")
    # Two genuinely crossing values with disjoint segment spans share a slot.
    def_pos, last_use = {0: 0, 1: 10}, {0: 7, 1: 15}
    slot_of, n = _alloc_slots(def_pos, last_use, seg_of)
    check(n == 1, "disjoint lifetimes share a slot")
    # Overlapping spans must not.
    def_pos, last_use = {0: 0, 1: 7}, {0: 12, 1: 15}
    slot_of, n = _alloc_slots(def_pos, last_use, seg_of)
    check(n == 2, "overlapping lifetimes get distinct slots")


def test_bool_typing() -> None:
    print("test_bool_typing")
    check(_is_bool_rhs("v1 <= zeta_threshold"), "a comparison is a bool")
    check(_is_bool_rhs("v1 < -v2"), "a comparison against a negation is a bool")
    check(_is_bool_rhs("v1 <= dens_threshold || v2"), "a disjunction is a bool")
    # piecewise3 takes a bool and returns an f64: the inner comparison must not
    # make the definition itself a bool. This is the shape that broke the first
    # emitted tree.
    check(not _is_bool_rhs("piecewise3(v1 <= zeta_threshold, v2, 0.3e1)"),
          "piecewise3 over a comparison is an f64")
    check(not _is_bool_rhs("v1 * v2 + 0.2e1"), "arithmetic is an f64")
    check(not _is_bool_rhs("f64::sqrt(v1)"), "a math call is an f64")
    types = _value_types([
        "        let v0 = rho[ip];",
        "        let v1 = v0 <= zeta_threshold;",
        "        let v2 = piecewise3(v1, v0, 0.1e1);",
    ])
    check(types == {0: "f64", 1: "bool", 2: "f64"}, "value types classified")


def bool_chain(n: int) -> list[str]:
    """A chain whose comparison result is consumed only at the very end, so the
    bool must cross every cut."""
    stmts = ["        let v0 = rho[ip];",
             "        let v1 = v0 <= zeta_threshold;"]
    for i in range(2, n):
        stmts.append(f"        let v{i} = v{i - 1} * 1.5;" if i > 2
                     else f"        let v{i} = v0 * 1.5;")
    stmts.append(f"        let v{n} = piecewise3(v1, v{n - 1}, 0.1e1);")
    stmts.append(f"        zk[ip] += v{n};")
    return stmts


def test_bool_scratch() -> None:
    print("test_bool_scratch")
    out = bool_chain(4000)
    plan = plan_segments(out, 1000)
    check(plan["nbools"] == 1, "the crossing bool gets a slot in its own array")
    check(plan["slot_of"][1] == ("wb", 0), "the bool is routed to the bool array")
    sig = ("pub fn k(\n    rho: &[f64],\n    zk: &mut [f64],\n"
           "    zeta_threshold: f64,\n) {")
    params = [("rho", "&[f64]"), ("zk", "&mut [f64]"),
              ("zeta_threshold", "f64")]
    text = emit_segmented("k", sig, params, out, plan, "zk.len()")
    check("let mut wb_vec = vec![false; 1];" in text,
          "the bool scratch array is emitted with a bool initialiser")
    check("let wb: &mut [bool; 1] = (&mut wb_vec[..]).try_into().unwrap();" in text,
          "the bool scratch array is typed [bool; N]")
    check("wb[0] = v1;" in text, "the bool is written to the bool array")
    check("let v1 = wb[0];" in text, "the bool is read back from the bool array")
    check("w[0] = v1;" not in text, "the bool never enters the f64 array")
    mods = seg_mods(text)
    check(all("w: &mut [f64; 1]" in m and "wb: &mut [bool; 1]" in m
              for m in mods), "every segment threads both scratch arrays")


def test_plan_thresholds() -> None:
    print("test_plan_thresholds")
    check(plan_segments(chain(100), 0) is None, "seg_target 0 disables planning")
    check(plan_segments(chain(100), 1000) is None, "small output stays whole")
    plan = plan_segments(chain(4000), 1000)
    check(plan is not None, "large output is planned")
    check(plan["defs"] == 4000, "def count reported")
    nseg = len(plan["bounds"]) - 1
    check(nseg == 4, f"4000 defs at target 1000 gives 4 segments (got {nseg})")
    check(plan["nslots"] == 1, "a linear chain crosses with a single slot")
    check(plan["nbools"] == 0, "a chain with no comparison needs no bool slots")
    huge = plan_segments(chain(200_000), 1000)
    check(len(huge["bounds"]) - 1 == 16, "segment count is capped at 16")


def test_bounds_partition() -> None:
    print("test_bounds_partition")
    out = chain(5000)
    plan = plan_segments(out, 1000)
    b = plan["bounds"]
    check(b[0] == 0 and b[-1] == len(out), "bounds span the statement list")
    check(all(b[i] < b[i + 1] for i in range(len(b) - 1)),
          "bounds are strictly increasing")
    check(len(plan["seg_of"]) == len(out), "every statement is assigned")


def _params() -> list[tuple[str, str]]:
    return [("rho", "&[f64]"), ("zk", "&mut [f64]"),
            ("dens_threshold", "f64")]


def seg_mods(text: str) -> list[str]:
    """The `mod segN { ... }` blocks of an emission, without the wrapper."""
    return text[:text.index("\npub fn ")].split("mod seg")[1:]


def _emit(n: int, target: int) -> str:
    out = chain(n)
    plan = plan_segments(out, target)
    sig = ("pub fn k(\n    rho: &[f64],\n    zk: &mut [f64],\n"
           "    dens_threshold: f64,\n) {")
    return emit_segmented("k", sig, _params(), out, plan, "zk.len()")


def test_emit_shape() -> None:
    print("test_emit_shape")
    text = _emit(4000, 1000)
    check(text.count("mod seg") == 4, "one module per segment")
    check(text.count("#[inline(never)]") == 4, "every segment is inline(never)")
    check("pub(super) fn run(" in text and "pub fn run(" not in text,
          "segments are pub(super) so harnesses do not treat them as entries")
    check(text.count("pub fn k(") == 1, "exactly one public entry point")
    check("let mut w_vec = vec![0.0f64; 1];" in text,
          "scratch is heap-allocated and one slot wide for a linear chain")
    check("let w: &mut [f64; 1] = (&mut w_vec[..]).try_into().unwrap();" in text,
          "scratch is a fixed-size array reference")
    check(text.index("w_vec") < text.index("for ip in"),
          "scratch is allocated outside the grid loop")
    check("for ip in 0..zk.len() {" in text, "loop bound preserved")
    # Statement order and content must survive untouched.
    body_defs = re.findall(r"let v(\d+) = v(\d+) \* 1\.5;", text)
    check(len(body_defs) == 3999, "every original definition is emitted once")
    check(all(int(a) == int(b) + 1 for a, b in body_defs),
          "definitions keep their original order and operands")


def test_emit_scratch_traffic() -> None:
    print("test_emit_scratch_traffic")
    text = _emit(4000, 1000)
    reads = re.findall(r"let v(\d+) = w\[(\d+)\];", text)
    writes = re.findall(r"w\[(\d+)\] = v(\d+);", text)
    check(len(writes) == 3, "one scratch write per cut for a linear chain")
    check(len(reads) == 3, "one scratch read per cut for a linear chain")
    check({v for v, _ in reads} == {v for _, v in writes},
          "every scratch read matches a scratch write of the same value")
    # A read must never precede the write of its slot.
    for slot in {s for _, s in reads}:
        first_read = text.index(f"= w[{slot}];")
        first_write = text.index(f"w[{slot}] = v")
        check(first_write < first_read, f"slot {slot} is written before it is read")


def test_emit_param_narrowing() -> None:
    print("test_emit_param_narrowing")
    text = _emit(4000, 1000)
    mods = seg_mods(text)
    check(len(mods) == 4, "four segment modules recovered")
    # Only the final segment stores to zk, so only it takes zk.
    check(sum(1 for m in mods if "zk: &mut [f64]" in m) == 1,
          "only the storing segment takes the output buffer")
    check(sum(1 for m in mods if "rho: &[f64]" in m) == 1,
          "only the loading segment takes the input buffer")
    check(all("ip: usize" in m and "w: &mut [f64; 1]" in m for m in mods),
          "every segment takes ip and scratch")
    check(not any("dens_threshold: f64" in m for m in mods),
          "an unreferenced scalar is dropped from every segment")


def test_stride_bound_preserved() -> None:
    print("test_stride_bound_preserved")
    out = chain(4000, out_name="v4rho4", stride=5)
    plan = plan_segments(out, 1000)
    sig = ("pub fn k(\n    rho: &[f64],\n    v4rho4: &mut [f64],\n"
           "    dens_threshold: f64,\n) {")
    params = [("rho", "&[f64]"), ("v4rho4", "&mut [f64]"),
              ("dens_threshold", "f64")]
    text = emit_segmented("k", sig, params, out, plan, "v4rho4.len() / 5")
    check("for ip in 0..v4rho4.len() / 5 {" in text,
          "a dim>1 output keeps its strided loop bound")
    check("v4rho4[ip * 5] += v3999;" in text, "the strided store is unchanged")


def test_reject_value_shaped_param() -> None:
    print("test_reject_value_shaped_param")
    out = chain(4000)
    plan = plan_segments(out, 1000)
    sig = "pub fn k(\n    v1: &[f64],\n    zk: &mut [f64],\n) {"
    try:
        emit_segmented("k", sig, [("v1", "&[f64]"), ("zk", "&mut [f64]")],
                       out, plan, "zk.len()")
        check(False, "a parameter named like a value id is rejected")
    except MergeError:
        check(True, "a parameter named like a value id is rejected")


def _split_files() -> dict[str, str]:
    """A minimal two-part split output, as merge_texts expects to find it."""
    head = ("#![allow(unused)]\n"
            "use libxc_rkernel_math::constants::{M_PI};\n\n")
    mod_rs = (head + "pub fn k_all(\n    rho: &[f64],\n    zk: &mut [f64],\n"
              "    vrho: &mut [f64],\n) {\n"
              "    k_part0(rho, zk);\n    k_part1(rho, vrho);\n}\n")
    part0 = (head + "pub fn k_part0(\n    rho: &[f64],\n    zk: &mut [f64],\n"
             ") {\n    for ip in 0..zk.len() {\n")
    part1 = (head + "pub fn k_part1(\n    rho: &[f64],\n    vrho: &mut [f64],\n"
             ") {\n    for ip in 0..vrho.len() {\n")
    # Both parts recompute the same 3000-long chain: the merge must keep one.
    shared = ["        let t0 = rho[ip];"]
    for i in range(1, 3000):
        shared.append(f"        let t{i} = t{i - 1} * 1.5;")
    part0 += "\n".join(shared) + "\n        zk[ip] += t2999;\n    }\n}\n"
    part1 += "\n".join(shared) + "\n        vrho[ip] += t2999;\n    }\n}\n"
    return {"mod.rs": mod_rs, "part0.rs": part0, "part1.rs": part1}


def test_merge_texts_end_to_end() -> None:
    print("test_merge_texts_end_to_end")
    files = _split_files()
    whole, s_whole = merge_texts("k", files)
    segd, s_seg = merge_texts("k", files, seg_target=1000)
    check(s_whole["defs_out"] == s_seg["defs_out"],
          f"segmentation does not change the value count ({s_seg['defs_out']})")
    check(s_whole["defs_in"] == 6000 and s_whole["defs_out"] == 3000,
          "the merge still deduplicates across parts")
    check(s_whole["stores"] == s_seg["stores"] == 2, "store count preserved")
    check(s_seg["segments"] == 3, f"3 segments emitted (got {s_seg['segments']})")
    check("mod seg0" in segd and "mod seg0" not in whole,
          "only the segmented emission has modules")
    # The arithmetic text must be identical between the two emissions.
    def defs(t: str) -> list[str]:
        return re.findall(r"let (v\d+ = [^;]*\* 1\.5);", t)
    check(defs(whole) == defs(segd),
          "every arithmetic definition is textually identical to the whole form")
    order = re.findall(r"(zk|vrho)\[ip\] \+=", segd)
    check(order == ["zk", "vrho"], "store order follows part order")


def test_seg_target_ignored_with_cap() -> None:
    print("test_seg_target_ignored_with_cap")
    files = _split_files()
    text, stats = merge_texts("k", files, cap=1000, seg_target=1000)
    check(stats["segments"] == 1 and stats["groups"] > 1,
          "cap and seg-target do not combine; cap wins")
    check("mod seg" not in text, "capped emission has no segment modules")


def test_cap_keeps_committed_shape() -> None:
    print("test_cap_keeps_committed_shape")
    # `cap` groups stay sibling free functions in one module. That means they
    # share a codegen unit and buy no build parallelism -- but emitting them as
    # separate `#[inline(never)]` modules was measured far worse still, because
    # it costs the SLP vectorisation the merged basic block earns (see the
    # segmentation note in this file's module docstring).
    text, stats = merge_texts("k", _split_files(), cap=1000)
    check(stats["groups"] > 1, f"the cap produced {stats['groups']} groups")
    check("mod grp" not in text, "capped groups are not wrapped in modules")
    check("#[inline(never)]" not in text, "capped groups are not inline(never)")
    check(text.count("for ip in 0..") == stats["groups"],
          "every capped group keeps its own grid loop")
    check("pub fn k_all(" in text, "the public entry point is preserved")


def main() -> int:
    for fn in sorted(
        (v for k, v in globals().items() if k.startswith("test_")),
        key=lambda f: f.__code__.co_firstlineno,
    ):
        fn()
    print()
    if FAILED:
        print(f"{len(FAILED)} FAILED:")
        for f in FAILED:
            print(f"  - {f}")
        return 1
    print("all checks passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
