# What the translator can still do for kernel speed

Follow-up to `docs/perf/vs-libxc.md`, which took the rayon backend to 2.4–4.7x
faster than caller-parallelised libxc. This asks the next question: is there
anything left in `tools/translate_rayon/` that would make the *emitted kernels*
faster?

**Short answer: no, not without changing the numbers.** Five levers were
implemented and measured; four are worth ~0 and the fifth is already at its best
setting. The one remaining lever with real headroom is not in the translator at
all — it is the libm transcendentals — and it costs bit-exactness, so it needs a
decision rather than a patch. The measurements are below so none of this has to
be re-derived.

Box: AMD Ryzen AI 7 350 (Zen 5), 16 threads. All builds `-C target-cpu=native`.

## First, correct the record: the kernels already vectorise

`AGENTS.md` recorded that the merged kernels get "2-wide packed SSE from LLVM's
*SLP* vectoriser, not loop vectorisation: the grid loop is not unrolled (one
point per iteration)." **That was true of the pre-`target-cpu` build and is no
longer true.** With `target-cpu=native` the grid loop is vectorised 8-wide with
AVX-512. `gga_x_b88` vxc unpol, from the built rlib:

```
106 packed ops, all on %zmm      vmovupd (%r10,%rcx,8),%zmm3     <- 8 grid points
                                 vaddpd  (%r8,%rcx,8),%zmm1,%zmm13 <- zk[ip] += , vectorised
```

The indexed `(%reg,%reg,8)` loads and stores are the tell: those are eight
consecutive grid points, not a broadcast of one point's scalar. Across the built
tree, 30% of kernel functions loop-vectorise; the non-vectorised remainder is
dominated by the enormous `lxc_pol`/`kxc_pol` bodies (100k–400k instructions),
where 8 lanes cannot fit in 32 `zmm` registers.

Beware two easy mistakes when checking this with `objdump`:

* On an **archive**, output is split by `Disassembly of section`, not by
  `<symbol>:` blocks. Splitting on the latter silently reports zero instructions.
* Operand regexes like `[^,]*` do not survive a memory operand — `(%rdi,%rax,8)`
  contains commas. That turns "all packed ops are on `zmm`" into "there are no
  packed ops".

Both produced confidently wrong readings here before being caught.

## Where the time actually goes

Static analysis of the built vxc kernels (spill = `vmov* %zmm, (%rsp)`):

| kernel | insns | spill | reload | stack traffic | divisions | loop-vectorised |
|---|--:|--:|--:|--:|--:|:-:|
| `mgga_c_r2scan` | 3050 | 417 | 297 | **23 %** | 129 | yes |
| `mgga_x_scan` | 1771 | 188 | 141 | 19 % | 80 | yes |
| `gga_c_lyp` | 853 | 53 | 33 | 10 % | 45 | yes |
| `gga_x_b88` | 890 | 59 | 39 | 11 % | 34 | yes |
| `gga_x_rge2` | 703 | 10 | 9 | 3 % | 26 | yes |
| `lda_c_vwn` | 443 | 26 | 21 | 11 % | 27 | **no** |

So for the slowest kernel roughly a third of the cost is division throughput, a
quarter is spill traffic, and the rest is arithmetic. `gga_x_rge2` at 3 % shows
the spill traffic is not inherent to the arithmetic — it is register pressure
from how much `vnmerge` shares.

## The five levers

### 1. Eliminating slice bounds checks — worth ~0

The loop bound comes from one slice (`for ip in 0..zk.len()`) while other
parameters are indexed with the same `ip`, so LLVM cannot prove those in range:
`lda_c_vwn` alone carried 55 `panic_bounds_check` sites. Reslicing every
parameter to `np * stride` before the loop removes all of them (verified: 0 left
in the patched functions).

Measured on `gga_x_pbe` vxc in an isolated three-variant binary — same data, same
process, interleaved timing:

```
v0 as-emitted    3.94 ns/pt   1.00x
v1 reslice       3.92 ns/pt   1.01x
```

### 2. Hoisting loop-invariant values — worth ~0

`gga_x_pbe` vxc recomputes 19 values per grid point that depend only on
constants and `zeta_threshold`, including **three `pow_1_3` (cube root) calls**.
Hoisting them by hand:

```
v2 +hoist        4.00 ns/pt   0.98x
```

LLVM's LICM already does this. The emitted instruction count drops (538 vs 696)
because the hand-hoisted source gives LLVM less to clean up, but the loop body
was already identical.

All three variants fingerprinted identically (`ce0f244c739f0868`).

### 3. Register-pressure scheduling — worth ~0

Implemented as `tools/translate_rayon/sched.py`: greedy list scheduling over the
dependency DAG, taking at each step the statement that increases the live count
least. Value-preserving by construction (expressions untouched, only the order of
independent `let`s moves) and confirmed so by fingerprint.

| kernel | peak live | single-thread ns/pt |
|---|---|---|
| `mgga_c_r2scan` | 131 → 126 | 78.50 → 81.17 (0.97x) |
| `gga_x_b88` | 27 → 25 | 12.92 → 12.99 (0.99x) |
| `mgga_x_scan` | 55 → 54 | 27.47 → 27.27 (1.01x) |

The pass is **not wired into the pipeline**. Peak live falls 2–7 %, but the level
that matters is absolute: 131 live values against 32 `zmm` registers spills
either way. And LLVM re-schedules regardless — source order only biases the
initial IR. Kept as a diagnostic for the peak-live number.

### 4. Replacing `powf` with cbrt/sqrt chains — not applicable

2,316 `f64::powf` calls survive in the tree, which looks like a translation miss
given libxc's `util.h` defines `POW_1_3`/`POW_2_3`/`POW_4_3`/`POW_1_4`/`POW_3_2`
in terms of `cbrt` and `sqrt`. It is not one. The surviving exponents are
`1/6` (317), `-2.5` (233), `1/5` (145), `1/10`, `1/15`, `1/12`, `1/8` and various
`param_*` — none of them in the POW_n_3 family. Checked directly against the
source it was translated from:

```c
/* libxc-master/src/maple2c/gga_exc/gga_x_gg99.c */
pow(t56, 0.1e1 / 0.6e1)
```

libxc calls `pow` there too. The translation is faithful and there is no
bit-exact rewrite.

### 5. Vector width — already at the best setting

Zen 5 double-pumps some 512-bit operations, so narrower vectors can win on a
spill-heavy body. Tested on `mgga_c_r2scan` vxc, the worst case in the table:

| flags | registers used | ns/pt |
|---|---|--:|
| `target-cpu=native` | `zmm` (VF 8) | **81.35** |
| `native -C target-feature=+prefer-256-bit` | `ymm` (VF 4) | 88.14 |
| `target-cpu=x86-64-v3` | `ymm` (VF 4) | 85.26 |

All three fingerprinted identically. AVX-512 wins by 8 %, and the portable
`x86-64-v3` baseline costs 5 % — worth knowing for anyone who has to ship a
portable binary.

## What is left, and what it costs

The remaining headroom is **libm**. A `ln`, `exp`, `powf` or `atan` call is
opaque to the vectoriser; a loop containing one either stays scalar or vectorises
with the call scalarised into 8 separate invocations per iteration.
`lda_c_vwn` — 4 `ln` + 2 `atan` per point — is the only kernel in the table that
does not vectorise at all, and it is also the only case in `vs-libxc.md` where
this library does not beat libxc.

Ceiling measurement: the same `lda_c_vwn` body with `ln`/`atan` replaced by
vectorisable stand-ins (deliberately *wrong* results — this measures the ceiling,
nothing else):

```
real (libm ln/atan)                 69.76 ns/pt   loop not vectorised
ceiling (vectorisable stand-ins)    11.30 ns/pt   loop vectorised, 8-wide
```

**6.2x.** That is the prize, and it applies to the 78 % of kernel files that call
a transcendental.

Two ways to claim it, with very different costs:

* **Bit-exact: loop fission.** Keep calling glibc, but split the loop so the
  transcendental sits in its own scalar pass over a block of points and
  everything else vectorises. Results are unchanged — same calls, same order.
  But the transcendentals *are* the cost (~52 of `lda_c_vwn`'s 70 ns/pt is the
  six libm calls), so the ceiling here is only about **1.3x**, for a pass that
  needs scratch buffers sized by the live set across the cut. Probably not worth
  it.
* **Not bit-exact: a vectorisable `ln`/`exp` in `kernels-rayon/math`.** This is
  where the 6.2x lives. A sub-ulp implementation sits far inside the project's
  actual contract (1e-12 relative vs libxc).

  When this was written it was blocked by `rkverify` and the emitter's
  old-vs-new gate, both of which demanded bit-exactness against the CubeCL tree.
  **Those were removed on 2026-08-18**, in favour of gating emitter changes on
  the libxc oracle. The blocker is therefore gone, and this is now the
  highest-value open item in the tree.

Note the contract distinction that makes this a real option at all: `CLAUDE.md`
requires *energy relative error ≤ 1e-12 vs the libxc oracle*, and `AGENTS.md`
requires that *maple2c formula translations preserve floating-point operation
order*. Neither requires that a transcendental be glibc's. Bit-exactness was the
**verification method**, never the accuracy requirement -- which is why dropping
it in favour of oracle parity costs no accuracy guarantee.
