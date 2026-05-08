#!/usr/bin/env python3
"""Build the Plan 09-07 parity report from a cargo-test log.

Reads PARITY_TUPLE / PARITY_SUMMARY lines emitted by
verify/tests/parity_phase09.rs and writes a markdown report.

Usage:
    python3 tools/build_parity_report.py \
        --log log/cargo-test-09-parity-sweep.log \
        --out .planning/phases/09-reduce-kernel-build-time/09-07-PARITY-REPORT.md
"""
import argparse
import datetime
import os
import re
import subprocess
import sys


TUPLE_RE = re.compile(
    r"PARITY_TUPLE: (?P<name>\S+) (?P<id>\d+) (?P<order>\S+) (?P<spin>\S+) "
    r"(?:max_rel_err=(?P<rel>\S+)\s+)?(?P<status>PASS|FAIL|SKIP)(?:\s+(?P<reason>.+))?"
)
SUMMARY_RE = re.compile(
    r"PARITY_SUMMARY: (?P<bucket>\S+) total=(?P<total>\d+) "
    r"pass=(?P<pass>\d+) skip=(?P<skip>\d+) fail=(?P<fail>\d+)"
)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--log", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    if not os.path.isfile(args.log):
        print(f"log not found: {args.log}", file=sys.stderr)
        return 2

    gga_rows = []
    mgga_rows = []
    summaries = {}
    with open(args.log, "r", errors="replace") as f:
        for line in f:
            m = TUPLE_RE.search(line)
            if m:
                d = m.groupdict()
                row = {
                    "name": d["name"],
                    "id": d["id"],
                    "order": d["order"],
                    "spin": d["spin"],
                    "rel": d["rel"] or "",
                    "status": d["status"],
                    "reason": d["reason"] or "",
                }
                # Bucket by MGGA spot-check name prefix; everything else goes
                # to GGA. This works because the spot-check names start with
                # `mgga_`.
                if row["name"].startswith("mgga_"):
                    mgga_rows.append(row)
                else:
                    gga_rows.append(row)
                continue
            m = SUMMARY_RE.search(line)
            if m:
                summaries[m.group("bucket")] = {
                    "total": int(m.group("total")),
                    "pass": int(m.group("pass")),
                    "skip": int(m.group("skip")),
                    "fail": int(m.group("fail")),
                }

    head = (
        subprocess.run(
            ["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=False
        ).stdout.strip()
        or "unknown"
    )
    now = datetime.datetime.now().isoformat(timespec="seconds")

    gga = summaries.get("gga", {"total": 0, "pass": 0, "skip": 0, "fail": 0})
    mgga = summaries.get("mgga", {"total": 0, "pass": 0, "skip": 0, "fail": 0})
    total = gga["total"] + mgga["total"]
    pass_ = gga["pass"] + mgga["pass"]
    skip = gga["skip"] + mgga["skip"]
    fail = gga["fail"] + mgga["fail"]

    def render_table(rows):
        if not rows:
            return "_(no tuples emitted — see Skips and Deviations below)_"
        out = ["| Functional | ID | Order | Spin | Max rel_err | Status | Notes |",
               "|------------|----|-------|------|-------------|--------|-------|"]
        for r in rows:
            notes = r["reason"] if r["status"] != "PASS" else ""
            out.append(
                f"| {r['name']} | {r['id']} | {r['order']} | {r['spin']} | "
                f"{r['rel'] or '—'} | {r['status']} | {notes} |"
            )
        return "\n".join(out)

    skips = [r for r in (gga_rows + mgga_rows) if r["status"] == "SKIP"]
    fails = [r for r in (gga_rows + mgga_rows) if r["status"] == "FAIL"]

    skip_section = "_(none)_"
    if skips:
        out = ["| Functional | ID | Order | Spin | Reason |",
               "|------------|----|-------|------|--------|"]
        for r in skips:
            out.append(f"| {r['name']} | {r['id']} | {r['order']} | {r['spin']} | {r['reason']} |")
        skip_section = "\n".join(out)

    fail_section = "_(none)_"
    if fails:
        out = ["| Functional | ID | Order | Spin | Max rel_err | Detail |",
               "|------------|----|-------|------|-------------|--------|"]
        for r in fails:
            out.append(
                f"| {r['name']} | {r['id']} | {r['order']} | {r['spin']} | "
                f"{r['rel']} | {r['reason']} |"
            )
        fail_section = "\n".join(out)

    md = f"""# Phase 9 Plan 07 — Oracle Parity Sweep Report

**Date:** {now}
**Git HEAD:** {head}
**Tolerance:** strict 1e-12 (CONTEXT D-14)
**Source log:** `{args.log}`

## Summary

| Bucket | Total | Pass | Skip | Fail |
|--------|-------|------|------|------|
| GGA deferred sweep | {gga['total']} | {gga['pass']} | {gga['skip']} | {gga['fail']} |
| MGGA spot-check    | {mgga['total']} | {mgga['pass']} | {mgga['skip']} | {mgga['fail']} |
| **Total**          | **{total}** | **{pass_}** | **{skip}** | **{fail}** |

## Per-Functional Results (Deferred GGA Sweep)

{render_table(gga_rows)}

## MGGA Spot-Check Results

{render_table(mgga_rows)}

## Skips and Deviations

{skip_section}

## Failures

{fail_section}
"""

    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    with open(args.out, "w") as f:
        f.write(md)
    print(f"wrote {args.out} (gga={gga['total']}/{gga['pass']}, mgga={mgga['total']}/{mgga['pass']})")
    return 0 if fail == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
