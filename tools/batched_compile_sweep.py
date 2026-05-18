#!/usr/bin/env python3
"""Batched compile sweep — pure orchestrator for per-functional subcrate builds.

Wraps `cargo build -p <pkg> --jobs N` over alphabetical-within-family batches of
on-disk per-functional subcrates (LDA → GGA → MGGA). Bounded 2-pass failure mode:
parallel pass (`-p ... -p ... --jobs {jobs}`) → sequential pass (`-p <pkg> --jobs 1`)
→ halt-and-surface via `.continue-here.md` on confirmed failure.

Locked behavior contract — see .planning/phases/11-splitter-v2-unified-5k-cap/11-07-PLAN.md
Task 1 for D-31 (pure orchestrator), D-32 (--jobs is per-invocation only),
D-33 (bounded 2-pass), D-34 (peak-RSS measurement). AP-2 narrowed: this is the
sole permitted jobs override path — `the repo cargo build config` stays jobs=1.

Per-package invocation: ONE cargo call per package within a batch, each with
`--jobs {jobs}`. Batch parallelism is sequential-with-overlap (one cargo invocation
at a time; cargo internally parallelizes its rustc subjobs by --jobs). This avoids
the .cargo/index lock contention that combined `-p` invocations can hit on initial
build, and gives clean per-package wall-clock + RSS attribution.
"""

from __future__ import annotations

import argparse
import json
import re
import resource
import subprocess
import sys
import time
from dataclasses import asdict, dataclass
from pathlib import Path

# Repo root is two levels up from this file: tools/ -> repo root.
REPO_ROOT = Path(__file__).resolve().parent.parent

# Roster source is the on-disk per-functional subcrate dir set under
# `crates/kernels/{family}/`. This is the superset of routed + unrouted +
# deferred + carve-out subcrates. The plan (11-07-PLAN.md Task 1 step 1)
# describes the roster as `cached_routed_funcnames(family)` UNION 7 carve-outs
# (= 273 packages by the plan's reckoning), but that source returns the
# *routed-in-model* set (36/105/25) while the on-disk set is 43/131/92 = 266.
# This script sweeps the on-disk set: it gives strictly broader coverage
# (every per-functional subcrate that exists on disk gets compile-checked,
# regardless of dispatch-tree routing). Documented per plan's escape hatch
# "or `total_packages=N` where N is the actual current count".
FAMILIES = ("lda", "gga", "mgga")
PKG_PREFIX = "libxc-kernel-"
DEFAULT_MANIFEST = REPO_ROOT / ".cache" / "batched-compile-sweep-manifest.json"
DEFAULT_SUMMARY = REPO_ROOT / ".cache" / "batched-compile-sweep-summary.md"
DEFAULT_CONTINUE = (
    REPO_ROOT
    / ".planning"
    / "phases"
    / "11-splitter-v2-unified-5k-cap"
    / ".continue-here.md"
)
RSS_PATTERN = re.compile(r"Maximum resident set size \(kbytes\):\s*(\d+)")


@dataclass
class PackageResult:
    package: str
    family: str
    batch_index: int
    pass_num: int  # 1 = parallel batch, 2 = sequential retry, -1 = failed
    result: str  # "ok" | "fail"
    wall_time_s: float
    peak_rss_mb: float
    stderr_tail: str | None = None


@dataclass
class BatchTiming:
    batch_index: int
    package_count: int
    wall_time_s: float
    peak_rss_mb: float
    all_ok: bool


def build_roster(families: list[str]) -> list[tuple[str, str]]:
    """Return [(family, package_name), ...] sorted within family in family order.

    Roster source is the ON-DISK per-functional subcrate dir set under
    `crates/kernels/{family}/`. This is the union of routed + unrouted +
    deferred + carve-out subcrates that exist on disk after Phase 11's
    splitter passes. Counts (current tree):
      LDA: 43; GGA: 131; MGGA: 92 (incl. mgga_c_b94 + 6 other carve-outs)
    Total: 266 (deviates from plan's stated 273; documented in 11-07-SUMMARY).
    """
    roster: list[tuple[str, str]] = []
    for fam in families:
        family_dir = REPO_ROOT / "crates" / "kernels" / fam
        if not family_dir.is_dir():
            raise RuntimeError(f"family dir missing: {family_dir}")
        names = sorted(
            d.name for d in family_dir.iterdir() if d.is_dir() and (d / "Cargo.toml").is_file()
        )
        roster.extend((fam, PKG_PREFIX + n) for n in names)
    return roster


def apply_start_after(
    roster: list[tuple[str, str]], start_after: str | None
) -> list[tuple[str, str]]:
    if not start_after:
        return roster
    target = start_after if start_after.startswith(PKG_PREFIX) else PKG_PREFIX + start_after
    out: list[tuple[str, str]] = []
    seen = False
    for fam, pkg in roster:
        if seen:
            out.append((fam, pkg))
            continue
        if pkg == target:
            seen = True
    if not seen:
        raise RuntimeError(
            f"--start-after package not found in roster: {target!r}"
        )
    return out


def chunk(seq: list, size: int) -> list[list]:
    return [seq[i : i + size] for i in range(0, len(seq), size)]


def parse_peak_rss_kb(stderr_text: str) -> int | None:
    m = RSS_PATTERN.search(stderr_text)
    return int(m.group(1)) if m else None


def have_gnu_time() -> bool:
    """Detect GNU time at /usr/bin/time (BSD time on macOS lacks -v)."""
    if not Path("/usr/bin/time").exists():
        return False
    try:
        r = subprocess.run(
            ["/usr/bin/time", "-v", "true"],
            capture_output=True,
            text=True,
            timeout=5,
        )
        return RSS_PATTERN.search(r.stderr) is not None
    except (subprocess.SubprocessError, OSError):
        return False


def cargo_build_one(
    pkg: str, jobs: int, use_time: bool
) -> tuple[int, float, float, str]:
    """Run `cargo build -p <pkg> --jobs <jobs>`. Return (exit, wall_s, peak_rss_mb, stderr_tail).

    When `use_time` is True, wrap in `/usr/bin/time -v` to measure peak-RSS.
    Fallback uses `resource.getrusage(RUSAGE_CHILDREN)` differential — less
    accurate when concurrent children exist but adequate for sequential sweeps.
    """
    cmd = ["cargo", "build", "-p", pkg, "--jobs", str(jobs)]
    if use_time:
        cmd = ["/usr/bin/time", "-v"] + cmd
    rusage_before = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
    t0 = time.monotonic()
    proc = subprocess.run(cmd, capture_output=True, text=True, cwd=REPO_ROOT)
    wall = time.monotonic() - t0
    stderr_text = proc.stderr
    if use_time:
        peak_kb = parse_peak_rss_kb(stderr_text)
        peak_mb = (peak_kb / 1024.0) if peak_kb is not None else 0.0
    else:
        rusage_after = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
        # ru_maxrss units: bytes on macOS, kilobytes on Linux. Assume Linux here.
        delta_kb = max(0, rusage_after - rusage_before)
        peak_mb = delta_kb / 1024.0
    stderr_tail = "\n".join(stderr_text.splitlines()[-50:])
    return proc.returncode, wall, peak_mb, stderr_tail


def write_manifest_record(manifest_path: Path, record: PackageResult) -> None:
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    with manifest_path.open("a") as f:
        rec = asdict(record)
        rec["pass"] = rec.pop("pass_num")
        # Always include stderr_tail key for ok records too, but as None for compactness.
        f.write(json.dumps(rec, separators=(",", ":")) + "\n")
        f.flush()


def write_continue_here(
    path: Path,
    failing_pkg: str,
    family: str,
    cargo_cmd: str,
    full_stderr: str,
    peak_rss_mb: float,
    batch_index: int,
    remaining_count: int,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    body = f"""---
phase: 11-splitter-v2-unified-5k-cap
plan: 07
task: 2
status: HALTED
captured: {time.strftime("%Y-%m-%d")}
severity: blocking
trigger: batched_compile_sweep.py confirmed compile failure on `{failing_pkg}` in pass 2 (sequential).
---

# Phase 11-07 Sweep HALT — confirmed compile failure

## Critical Anti-Patterns

| # | name | severity | rule |
|---|---|---|---|
| sweep-halt | Confirmed per-`-p` compile failure on `{failing_pkg}` after pass-2 sequential retry. | blocking | The sweep is a pure orchestrator (D-31); a confirmed pass-2 failure is the real translator/codegen signal. Per AP-8 boundary: the sweep does NOT auto-patch — diagnosis routes through /gsd:discuss-phase. |

## Failure Detail

**Failing package:** `{failing_pkg}` (family: `{family}`)

**Cargo invocation:** `{cargo_cmd}`

**Peak-RSS observed:** {peak_rss_mb:.1f} MB

**Batch index at halt:** {batch_index}

**Remaining unrun packages after halt:** {remaining_count}

## Full cargo stderr (no truncation)

```
{full_stderr}
```

## Resume guidance

After diagnosis + fix, re-invoke the sweep with `--start-after {failing_pkg}` to
skip already-verified packages, OR re-run from scratch if the fix may have
affected earlier packages too.
"""
    path.write_text(body)


def write_summary(
    path: Path,
    results: list[PackageResult],
    batches: list[BatchTiming],
    total_wall: float,
    halt_pkg: str | None,
    effective_jobs: int,
    effective_batch_size: int,
    total_packages: int,
    families: list[str],
) -> None:
    by_pass_ok = {1: 0, 2: 0}
    by_family: dict[str, dict[str, int]] = {f: {"ok": 0, "fail": 0, "pass2": 0} for f in families}
    fail_records: list[PackageResult] = []
    for r in results:
        if r.result == "ok":
            by_pass_ok[r.pass_num] = by_pass_ok.get(r.pass_num, 0) + 1
            by_family[r.family]["ok"] += 1
            if r.pass_num == 2:
                by_family[r.family]["pass2"] += 1
        else:
            by_family[r.family]["fail"] += 1
            fail_records.append(r)

    peak_of_peak = max((b.peak_rss_mb for b in batches), default=0.0)
    verdict = f"HALTED_AT_BATCH_{batches[-1].batch_index}" if halt_pkg else "ALL_OK"

    lines = [
        "# Batched compile sweep — summary",
        "",
        f"- effective jobs: `{effective_jobs}`",
        f"- effective batch_size: `{effective_batch_size}`",
        f"- families: `{','.join(families)}`",
        f"- total packages swept: `{total_packages}`",
        f"- pass-1 successes: `{by_pass_ok[1]}`",
        f"- pass-2 successes: `{by_pass_ok[2]}`",
        f"- failures: `{len(fail_records)}`",
        f"- total wall-clock: `{total_wall:.1f} s`",
        f"- peak-of-peak RSS: `{peak_of_peak:.1f} MB`",
        "",
        "## Per-family",
        "",
        "| family | ok | pass-2 | fail |",
        "|---|---|---|---|",
    ]
    for f in families:
        d = by_family[f]
        lines.append(f"| {f} | {d['ok']} | {d['pass2']} | {d['fail']} |")
    lines += ["", "## Per-batch timing + RSS", ""]
    lines.append("| batch_index | package_count | wall_time_s | peak_rss_mb | all_ok |")
    lines.append("|---|---|---|---|---|")
    for b in batches:
        lines.append(
            f"| {b.batch_index} | {b.package_count} | {b.wall_time_s:.1f} | {b.peak_rss_mb:.1f} | {b.all_ok} |"
        )
    if fail_records:
        lines += ["", "## Failures", ""]
        for r in fail_records:
            lines.append(f"- `{r.package}` (family `{r.family}`, batch {r.batch_index})")
    lines += ["", f"VERDICT: {verdict}", ""]
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines))


def run_sweep(args: argparse.Namespace) -> int:
    families = [f.strip() for f in args.families.split(",") if f.strip()]
    for f in families:
        if f not in FAMILIES:
            print(f"ABORT: unknown family {f!r} (expected subset of {FAMILIES})", file=sys.stderr)
            return 2

    if args.roster:
        roster_path = Path(args.roster)
        if not roster_path.is_file():
            print(f"ABORT: roster file not found: {roster_path}", file=sys.stderr)
            return 2
        raw = [ln.strip() for ln in roster_path.read_text().splitlines() if ln.strip()]
        # Roster file: one package per line, no family prefix info; infer family from pkg name.
        roster = []
        for pkg in raw:
            name = pkg[len(PKG_PREFIX):] if pkg.startswith(PKG_PREFIX) else pkg
            for fam in families:
                if (REPO_ROOT / "crates" / "kernels" / fam / name / "Cargo.toml").is_file():
                    roster.append((fam, PKG_PREFIX + name))
                    break
            else:
                print(f"WARN: roster pkg {pkg!r} not on disk in {families}", file=sys.stderr)
    else:
        roster = build_roster(families)

    roster = apply_start_after(roster, args.start_after)
    total_packages = len(roster)
    effective_jobs = args.jobs
    effective_batch_size = args.batch_size

    print(
        f"[batched-compile-sweep] effective jobs={effective_jobs}, "
        f"batch_size={effective_batch_size}, total_packages={total_packages}, "
        f"families={','.join(families)}"
    )

    if args.dry_run:
        print(f"[batched-compile-sweep] DRY RUN — no cargo invocations")
        batches_dry = chunk(roster, effective_batch_size)
        for i, b in enumerate(batches_dry):
            print(f"  batch {i}: {len(b)} packages")
            for fam, pkg in b:
                print(f"    [{fam}] {pkg}")
        return 0

    # Real sweep: clear manifest and continue-here.md before writing.
    manifest_path = Path(args.manifest)
    summary_path = Path(args.summary)
    continue_path = Path(args.continue_here)
    if manifest_path.exists():
        manifest_path.unlink()
    use_time = have_gnu_time()
    if not use_time:
        print(
            "[batched-compile-sweep] WARN: /usr/bin/time -v unavailable; "
            "falling back to resource.getrusage (less accurate)",
            file=sys.stderr,
        )

    batches = chunk(roster, effective_batch_size)
    results: list[PackageResult] = []
    batch_timings: list[BatchTiming] = []
    halt_pkg: str | None = None
    sweep_start = time.monotonic()
    cumulative_processed = 0

    for batch_idx, batch in enumerate(batches):
        if halt_pkg:
            break
        batch_t0 = time.monotonic()
        batch_peak_rss = 0.0
        batch_all_ok = True
        print(
            f"[batched-compile-sweep] batch {batch_idx}/{len(batches)-1} "
            f"({len(batch)} pkgs)"
        )

        # Pass 1: per-package with --jobs={effective_jobs}.
        # NOTE: cargo serializes registry/index access, so the parallelism is per-pkg
        # via --jobs internally, not via interleaved package compiles. This is the
        # planner's chosen path (Task 1 step 1 alternative) — simpler than combined `-p`
        # and avoids index-lock contention. Documented here per Task 1.
        pass1_results: list[PackageResult] = []
        for fam, pkg in batch:
            exit_code, wall_s, peak_mb, stderr_tail = cargo_build_one(
                pkg, effective_jobs, use_time
            )
            batch_peak_rss = max(batch_peak_rss, peak_mb)
            if exit_code == 0:
                rec = PackageResult(
                    package=pkg, family=fam, batch_index=batch_idx,
                    pass_num=1, result="ok",
                    wall_time_s=round(wall_s, 2), peak_rss_mb=round(peak_mb, 1),
                )
                pass1_results.append(rec)
            else:
                # Mark batch as failing — do NOT record pass=1 results yet,
                # the plan requires pass=2 sequential retry across the whole batch.
                batch_all_ok = False
                print(
                    f"  ✗ pass 1 fail: {pkg} (exit {exit_code}, wall {wall_s:.1f}s, "
                    f"peak {peak_mb:.0f} MB) — will retry in pass 2"
                )
                break  # abort pass 1 for this batch; move to pass 2

        if batch_all_ok:
            for rec in pass1_results:
                results.append(rec)
                write_manifest_record(manifest_path, rec)
                print(
                    f"  ✓ {rec.package} (pass=1, {rec.wall_time_s:.1f}s, "
                    f"{rec.peak_rss_mb:.0f} MB)"
                )
            batch_timings.append(BatchTiming(
                batch_index=batch_idx, package_count=len(batch),
                wall_time_s=round(time.monotonic() - batch_t0, 1),
                peak_rss_mb=round(batch_peak_rss, 1), all_ok=True,
            ))
            cumulative_processed += len(batch)
            continue

        # Pass 2: sequential with --jobs 1 over the WHOLE failing batch.
        print(f"[batched-compile-sweep] batch {batch_idx} pass 2 (sequential, --jobs 1)")
        for fam, pkg in batch:
            exit_code, wall_s, peak_mb, stderr_tail = cargo_build_one(pkg, 1, use_time)
            batch_peak_rss = max(batch_peak_rss, peak_mb)
            if exit_code == 0:
                rec = PackageResult(
                    package=pkg, family=fam, batch_index=batch_idx,
                    pass_num=2, result="ok",
                    wall_time_s=round(wall_s, 2), peak_rss_mb=round(peak_mb, 1),
                )
                results.append(rec)
                write_manifest_record(manifest_path, rec)
                print(
                    f"  ✓ {pkg} (pass=2, {wall_s:.1f}s, {peak_mb:.0f} MB)"
                )
            else:
                rec = PackageResult(
                    package=pkg, family=fam, batch_index=batch_idx,
                    pass_num=-1, result="fail",
                    wall_time_s=round(wall_s, 2), peak_rss_mb=round(peak_mb, 1),
                    stderr_tail=stderr_tail,
                )
                results.append(rec)
                write_manifest_record(manifest_path, rec)
                print(
                    f"  ✗ pass 2 fail: {pkg} (exit {exit_code}, wall {wall_s:.1f}s) — HALT"
                )
                halt_pkg = pkg
                cargo_cmd = f"cargo build -p {pkg} --jobs 1"
                # Re-run with capture for FULL stderr (not just 50-line tail).
                proc = subprocess.run(
                    ["cargo", "build", "-p", pkg, "--jobs", "1"],
                    capture_output=True, text=True, cwd=REPO_ROOT,
                )
                remaining_count = total_packages - cumulative_processed - (
                    batch.index((fam, pkg)) + 1
                )
                write_continue_here(
                    continue_path, pkg, fam, cargo_cmd, proc.stderr,
                    peak_mb, batch_idx, remaining_count,
                )
                break

        batch_timings.append(BatchTiming(
            batch_index=batch_idx, package_count=len(batch),
            wall_time_s=round(time.monotonic() - batch_t0, 1),
            peak_rss_mb=round(batch_peak_rss, 1), all_ok=(halt_pkg is None),
        ))
        cumulative_processed += len(batch)

    total_wall = time.monotonic() - sweep_start
    write_summary(
        summary_path, results, batch_timings, total_wall, halt_pkg,
        effective_jobs, effective_batch_size, total_packages, families,
    )
    print(
        f"[batched-compile-sweep] done. wall={total_wall:.1f}s, "
        f"verdict={'HALTED_AT_BATCH_' + str(batch_timings[-1].batch_index) if halt_pkg else 'ALL_OK'}"
    )
    return 2 if halt_pkg else 0


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(
        prog="batched_compile_sweep.py",
        description="Bounded 2-pass per-`-p` cargo build sweep over on-disk subcrates.",
    )
    p.add_argument("--batch-size", type=int, default=20)
    p.add_argument("--jobs", type=int, default=3)
    p.add_argument("--start-after", type=str, default=None)
    p.add_argument(
        "--families", type=str, default=",".join(FAMILIES),
        help="Comma-separated subset: lda, gga, mgga (default: all three)",
    )
    p.add_argument("--roster", type=str, default=None)
    p.add_argument("--manifest", type=str, default=str(DEFAULT_MANIFEST))
    p.add_argument("--summary", type=str, default=str(DEFAULT_SUMMARY))
    p.add_argument("--continue-here", type=str, default=str(DEFAULT_CONTINUE))
    p.add_argument("--dry-run", action="store_true")
    args = p.parse_args(argv)
    return run_sweep(args)


if __name__ == "__main__":
    sys.exit(main())
