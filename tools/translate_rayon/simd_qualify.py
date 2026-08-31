#!/usr/bin/env python3
"""Qualify `(functional, order, spin)` triples for the explicit-SIMD allowlist.

The gate is the one `docs/perf/simd-kernels.md` settled on once every
transcendental gained a bit-exact `libxc_rkernel_math::simd` form: **the
fingerprint must not move, and the kernel must get faster.** The accuracy half
is therefore exact rather than a tolerance, and the old "at least 1.5x" bar is
retired -- any measured speedup with an unchanged fingerprint qualifies.

Why this is a driver and not a checklist
----------------------------------------
There are 426 routed unpolarized exc/vxc candidates alone
(`candidate_profiler.py`), and each verdict needs a regeneration, a release
build and a timed run. Done one at a time that is days of wall-clock, most of
it spent relinking `libxc-reval` and `xcqual` -- a fixed cost that has nothing
to do with the candidate.

So candidates are tried in **batches**. A batch enables N triples at once, pays
the build cost once, and then times all N in a single `xcqual` process. No
bisection is needed when a batch is mixed: `xcqual` reports a fingerprint and a
timing per case, so every triple in the batch gets its own verdict from the one
run. Triples cannot interfere with each other -- they are separate functions in
separate crates -- so a batched verdict is the same verdict the triple would
have got alone.

The allowlist itself is never edited during a sweep. Batches are applied
through the `LIBXC_RS_SIMD_EXTRA` environment variable that `from_maple.py`
reads, so an interrupted sweep leaves the tree exactly as it found it. Writing
the winners into `SIMD_EXACT_FUNCS` is a separate, deliberate step (`--apply`).

The ledger (`docs/perf/simd-ledger.json`) records every verdict, accept and
reject alike, with the numbers behind it. A rejection is a result: it stops the
triple being retried blind later. Re-running the driver skips anything already
in the ledger, so a sweep is resumable after an interrupt.

Usage
-----
    # See what would be tried, in order, without building anything.
    python3 tools/translate_rayon/simd_qualify.py --tier 1 --dry-run

    # Sweep tier 1 (routed, unpolarized, exc/vxc, >=2 libm calls).
    python3 tools/translate_rayon/simd_qualify.py --tier 1 --batch 12

    # Write the accepted triples into from_maple.py's SIMD_EXACT_FUNCS.
    python3 tools/translate_rayon/simd_qualify.py --apply
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

import candidate_profiler
import from_maple

REPO = Path(__file__).resolve().parents[2]
LEDGER = REPO / "docs" / "perf" / "simd-ledger.json"

# Functionals a real DFT workload actually spends its time in. These go first
# within a tier, so an interrupted sweep has still done the useful part.
HOT = [
    "mgga_x_r2scan", "mgga_c_r2scan", "mgga_x_rscan", "mgga_c_rscan",
    "mgga_x_scan", "mgga_c_scan", "mgga_x_tpss", "mgga_c_tpss",
    "mgga_c_revtpss", "mgga_x_revtpss", "mgga_x_task", "mgga_x_m11",
    "gga_c_pbe", "gga_x_pbe", "gga_c_lyp", "gga_x_b88", "gga_c_pw91",
    "gga_x_pw91", "gga_c_p86vwn", "gga_c_pbe_vwn", "gga_c_q2d",
    "lda_c_pw", "lda_c_vwn", "lda_c_pz",
    "hyb_gga_xc_wb97", "hyb_mgga_xc_wb97mv", "mgga_xc_b97mv",
]

# Families whose `xcqual` support stops at order 2; see `qual_mgga` there.
MGGA_MAX_ORDER = ("exc", "vxc", "fxc")


# ---------------------------------------------------------------------------
# Candidate selection
# ---------------------------------------------------------------------------

def candidates(tier: int, records: list[dict]) -> list[dict]:
    """Triples to try, most promising first.

    Tier 1 is the profiler's own P1 queue: routed, unpolarized, `exc`/`vxc`, no
    scalar helper, at least two libm calls and all of them exact-translatable.
    Tiers 2 and 3 reach into P3 (which the profiler assigns on spin/order alone)
    and re-apply P1's substantive filters, because P3 mixes "polarized" together
    with "fourth derivative" and only the first is cheap to win.
    """
    out = []
    for r in records:
        if not r["routed"] or r["helpers"] or not r["all_exact"]:
            continue
        if r["tot_calls"] < 2:
            continue
        if (r["func"], r["order"], r["spin"]) in from_maple.SIMD_EXACT_FUNCS:
            continue
        if r["fam"] == "mgga" and r["order"] not in MGGA_MAX_ORDER:
            continue
        if tier == 1:
            ok = r["spin"] == "unpol" and r["order"] in ("exc", "vxc")
        elif tier == 2:
            ok = r["spin"] == "pol" and r["order"] in ("exc", "vxc")
        elif tier == 3:
            ok = r["spin"] == "unpol" and r["order"] == "fxc"
        elif tier == 4:
            ok = r["order"] in ("kxc", "lxc")
        else:
            raise SystemExit(f"unknown tier {tier}")
        if ok:
            out.append(r)

    # Hot functionals first, then by estimated transcendental cost: the two
    # things that predict a win, in the order the plan cares about them.
    def key(r):
        hot = HOT.index(r["func"]) if r["func"] in HOT else len(HOT)
        return (hot, -r["est_cost_ns"], r["func"], r["order"])

    return sorted(out, key=key)


# ---------------------------------------------------------------------------
# Ledger
# ---------------------------------------------------------------------------

def load_ledger() -> dict:
    if LEDGER.exists():
        return json.loads(LEDGER.read_text())
    return {}


def save_ledger(led: dict) -> None:
    LEDGER.parent.mkdir(parents=True, exist_ok=True)
    LEDGER.write_text(json.dumps(led, indent=1, sort_keys=True) + "\n")


def key_of(r: dict) -> str:
    return f"{r['func']}:{r['order']}:{r['spin']}"


def case_of(r: dict) -> str:
    """The `fam:name:order:spin` string `xcqual` takes."""
    return f"{r['fam']}:{r['func']}:{r['order']}:{r['spin']}"


# ---------------------------------------------------------------------------
# Shelling out
# ---------------------------------------------------------------------------

def run(cmd: list[str], env: dict | None = None, quiet: bool = False) -> str:
    e = dict(os.environ)
    if env:
        e.update(env)
    p = subprocess.run(cmd, cwd=REPO, env=e, capture_output=True, text=True)
    if p.returncode != 0:
        sys.stderr.write(f"\n$ {' '.join(cmd)}\n{p.stdout[-4000:]}{p.stderr[-4000:]}\n")
        raise RuntimeError(f"command failed: {cmd[0]}")
    if not quiet and p.stderr.strip():
        pass
    return p.stdout


def regen(funcs: list[str], extra: str) -> None:
    cmd = [sys.executable, "tools/translate_rayon/from_maple.py"]
    for f in sorted(set(funcs)):
        cmd += ["--func", f]
    run(cmd, env={"LIBXC_RS_SIMD_EXTRA": extra})


def restore(recs: list[dict]) -> None:
    """Put the kernel crates back exactly as git has them.

    Not `regen(funcs, "")`: the emitter's `#![allow(..)]` line has drifted from
    the committed tree's, so a plain regeneration restores the *behaviour* but
    rewrites the file, leaving cosmetic churn in the working tree for every
    functional the sweep touched. `git checkout` restores the bytes.
    """
    paths = sorted({f"crates/kernels-rayon/{r['fam']}/{r['func']}" for r in recs})
    run(["git", "checkout", "--"] + paths)


def build(jobs: int) -> float:
    t0 = time.time()
    run(["cargo", "build", "--release", "-p", "bench-vs-libxc",
         "--bin", "xcqual", "-j", str(jobs)])
    return time.time() - t0


QUAL_RE = re.compile(
    r"^QUAL (?P<case>\S+) np=(?P<np>\d+) ns1t=(?P<ns1t>[\d.]+) "
    r"nsNt=(?P<nsNt>[\d.]+) fp=(?P<fp>[0-9a-f]+) rejected=(?P<rej>\d+)"
    r"(?:\s+minforeign=(?P<mf>[\d.]+))?")


def xcqual_path() -> Path:
    exe = REPO / ".cache" / "cargo-target" / "release" / "xcqual"
    return exe if exe.exists() else REPO / "target" / "release" / "xcqual"


def measure(cases: list[str], np: int, reps: int, exe: Path | None = None) -> dict[str, dict]:
    exe = exe or xcqual_path()
    out = run([str(exe), "--quiet", "--np", str(np), "--reps", str(reps)] + cases)
    res = {}
    for line in out.splitlines():
        m = QUAL_RE.match(line.strip())
        if m:
            res[m["case"]] = {
                "ns1t": float(m["ns1t"]),
                "nsNt": float(m["nsNt"]),
                "fp": m["fp"],
                "rejected": int(m["rej"]),
                "minforeign": float(m["mf"]) if m["mf"] else None,
            }
        elif line.startswith("QUALSKIP"):
            parts = line.split()
            res[parts[1]] = {"skip": parts[2].split("=", 1)[1]}
    return res


# ---------------------------------------------------------------------------
# Applying the winners
# ---------------------------------------------------------------------------

def apply_accepted(led: dict) -> int:
    """Rewrite `SIMD_EXACT_FUNCS` in from_maple.py to include every accept."""
    src = (REPO / "tools" / "translate_rayon" / "from_maple.py").read_text()
    accepted = sorted(k for k, v in led.items() if v.get("verdict") == "accept")
    if not accepted:
        print("no accepted triples in the ledger; nothing to apply")
        return 0

    have = {f"{f}:{o}:{s}" for f, o, s in from_maple.SIMD_EXACT_FUNCS}
    new = [a for a in accepted if a not in have]
    if not new:
        print("every accepted triple is already in SIMD_EXACT_FUNCS")
        return 0

    lines = []
    for a in new:
        f, o, s = a.split(":")
        v = led[a]
        lines.append(f'    ("{f}", "{o}", "{s}"),'
                     f'  # {v["ratio"]:.2f}x  ({v["base_nsNt"]:.2f} -> {v["nsNt"]:.2f} ns/pt)')
    block = ("\n    # Added by tools/translate_rayon/simd_qualify.py; each line's\n"
             "    # ratio is sweep ns/pt before -> after, fingerprint unchanged.\n"
             + "\n".join(lines) + "\n}")
    # Anchored on the line that follows the set literal. If the file is
    # reshaped so this no longer matches, say so rather than reporting success
    # after a `replace` that quietly did nothing.
    anchor = "\n}\n\n# Sweep override"
    if anchor not in src:
        raise SystemExit(
            "simd_qualify --apply: cannot find the end of SIMD_EXACT_FUNCS in "
            "from_maple.py (expected the set literal to be followed by the "
            "'# Sweep override' comment). Update `anchor` in apply_accepted.")
    src = src.replace(anchor, block + "\n\n# Sweep override", 1)
    (REPO / "tools" / "translate_rayon" / "from_maple.py").write_text(src)
    print(f"added {len(new)} triples to SIMD_EXACT_FUNCS")
    return len(new)


# ---------------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--tier", type=int, default=1,
                    help="1 = unpol exc/vxc, 2 = pol exc/vxc, 3 = unpol fxc, 4 = kxc/lxc")
    ap.add_argument("--batch", type=int, default=12)
    ap.add_argument("--limit", type=int, default=0, help="0 = no limit")
    ap.add_argument("--np", type=int, default=100_000)
    ap.add_argument("--reps", type=int, default=5)
    ap.add_argument("--jobs", type=int, default=12)
    ap.add_argument("--family", default="", help="only this family (lda/gga/mgga)")
    ap.add_argument("--func", action="append", default=[],
                    help="only these functionals (repeatable)")
    ap.add_argument("--max-foreign", type=float, default=2.0,
                    help="if this many foreign cores were busy, record the "
                         "fingerprint but defer the speed verdict")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--apply", action="store_true",
                    help="write accepted triples into from_maple.py and exit")
    args = ap.parse_args()

    led = load_ledger()
    if args.apply:
        return 0 if apply_accepted(led) >= 0 else 1

    recs = candidate_profiler.profile_all()
    cands = candidates(args.tier, recs)
    if args.family:
        cands = [r for r in cands if r["fam"] == args.family]
    if args.func:
        cands = [r for r in cands if r["func"] in args.func]
    # A verdict that says something about the triple is final. One that says
    # something about the *machine* -- too busy to time, or no line came back --
    # is not, so those stay pending and are retried on the next run.
    PROVISIONAL = {"deferred-contention", "no-measurement"}
    decided = {k for k, v in led.items() if v.get("verdict") not in PROVISIONAL}
    pending = [r for r in cands if key_of(r) not in decided]
    n_done = len(cands) - len(pending)
    if args.limit:
        pending = pending[: args.limit]

    print(f"tier {args.tier}: {len(cands)} candidates, "
          f"{n_done} already in the ledger, "
          f"{len(pending)} to try, batch {args.batch}")
    if args.dry_run:
        for r in pending:
            print(f"  {case_of(r):<48} {r['tot_calls']:>3} calls  "
                  f"{r['est_cost_ns']:>6.1f} est ns  {r['calls']}")
        return 0
    if not pending:
        return 0

    # ---- Baseline: the tree as committed. One build, one timed run over
    # every pending case. This is also what proves a case is routed at all.
    print("\n=== baseline (committed allowlist) ===")
    bt = build(args.jobs)
    # Keep this binary. Each batch re-times the baseline with it immediately
    # beside the SIMD build, because the two are otherwise separated by a build
    # of several minutes -- long enough for the machine to get busy or quiet in
    # between, which corrupts the ratio even when each measurement looks fine on
    # its own. Measured: a baseline taken at 14.8 foreign cores against a SIMD
    # run taken on an idle box reported 26x for a kernel that is really ~1.9x.
    base_exe = REPO / ".cache" / "cargo-target" / "release" / "xcqual-baseline"
    shutil.copy2(xcqual_path(), base_exe)
    print(f"build {bt:.0f}s (baseline binary kept at {base_exe.name})")
    base = measure([case_of(r) for r in pending], args.np, args.reps)
    pending = [r for r in pending if "skip" not in base.get(case_of(r), {"skip": "nocase"})]
    print(f"{len(pending)} cases measured")

    # ---- Batches.
    for i in range(0, len(pending), args.batch):
        chunk = pending[i : i + args.batch]
        extra = ",".join(key_of(r) for r in chunk)
        funcs = [r["func"] for r in chunk]
        print(f"\n=== batch {i // args.batch + 1} "
              f"({i + 1}-{i + len(chunk)} of {len(pending)}) ===")
        for r in chunk:
            print(f"  {case_of(r)}")
        try:
            regen(funcs, extra)
            bt = build(args.jobs)
        except RuntimeError as e:
            # A batch that will not emit or will not compile is a real result
            # for every triple in it, but not one this driver can attribute, so
            # record it as a build failure and move on rather than guessing.
            print(f"  BUILD FAILED: {e}")
            for r in chunk:
                led[key_of(r)] = {"verdict": "build-failed", "case": case_of(r)}
            save_ledger(led)
            restore(chunk)
            build(args.jobs)
            continue
        cases = [case_of(r) for r in chunk]
        got = measure(cases, args.np, args.reps)
        # Same window, same machine state -- this is the comparison that counts.
        base = {**base, **measure(cases, args.np, args.reps, exe=base_exe)}
        print(f"  build {bt:.0f}s")

        for r in chunk:
            c = case_of(r)
            b, g = base.get(c), got.get(c)
            k = key_of(r)
            if not b or not g or "skip" in g:
                led[k] = {"verdict": "no-measurement", "case": c}
                continue
            same_fp = b["fp"] == g["fp"]
            ratio = b["nsNt"] / g["nsNt"] if g["nsNt"] > 0 else 0.0
            ratio1t = b["ns1t"] / g["ns1t"] if g["ns1t"] > 0 else 0.0
            foreign = max(b.get("minforeign") or 0.0, g.get("minforeign") or 0.0)
            if not same_fp:
                # Load cannot change the bits, so this verdict is always sound.
                verdict = "reject-fingerprint"
            elif foreign > args.max_foreign:
                # The bits are right but the clock was not: another process was
                # using the machine, so a ratio near 1 means nothing. Keep the
                # numbers and re-run this triple on a quiet box rather than
                # committing a coin-flip either way.
                verdict = "deferred-contention"
            elif ratio <= 1.0:
                verdict = "reject-slower"
            else:
                verdict = "accept"
            led[k] = {
                "verdict": verdict, "case": c,
                "base_ns1t": b["ns1t"], "ns1t": g["ns1t"], "ratio1t": round(ratio1t, 3),
                "base_nsNt": b["nsNt"], "nsNt": g["nsNt"], "ratio": round(ratio, 3),
                "base_fp": b["fp"], "fp": g["fp"], "fingerprint_same": same_fp,
                "calls": r["tot_calls"], "breakdown": r["calls"],
                "np": args.np, "reps": args.reps,
                # How busy the box was. A verdict taken with several foreign
                # cores running is a weak one; recorded so it can be re-run
                # rather than silently trusted.
                "minforeign": max(b.get("minforeign") or 0.0, g.get("minforeign") or 0.0),
            }
            flag = {"accept": "OK  ", "reject-slower": "SLOW",
                    "reject-fingerprint": "BITS",
                    "deferred-contention": "BUSY"}.get(verdict, "??  ")
            print(f"  {flag} {c:<46} {b['nsNt']:>7.2f} -> {g['nsNt']:>7.2f} ns/pt  "
                  f"{ratio:>5.2f}x  fp {'=' if same_fp else 'CHANGED'}")
        save_ledger(led)

        # Leave the tree in its committed state between batches, so an
        # interrupt never strands a candidate's SIMD form in the kernel tree.
        restore(chunk)

    n_acc = sum(1 for v in led.values() if v.get("verdict") == "accept")
    print(f"\nledger: {len(led)} verdicts, {n_acc} accepted -> {LEDGER}")
    print("run with --apply to write the accepted triples into from_maple.py")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
