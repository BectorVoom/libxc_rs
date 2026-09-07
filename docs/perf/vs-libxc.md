# Speed and memory against C libxc 7.0.0

Harness: `bench-vs-libxc` (`cargo run --release -p bench-vs-libxc --bin xcvs`).
Box: AMD Ryzen AI 7 350 (Zen 5), 16 threads, 30 GB.

Four legs per case:

| leg | what it is |
|---|---|
| `libxc-1t` | one `xc_*_exc_vxc` call — libxc's own API is serial |
| `libxc-Nt` | the grid cut into one chunk per core, one `xc_*` call each, over a rayon pool. **This is the bar.** It is what a DFT code gets from an OpenMP loop over grid batches; beating serial libxc with 16 threads would prove nothing |
| `rust-1t` | this library with splitting disabled (`min_chunk = usize::MAX`) — same core count, same libm, isolates kernel quality |
| `rust-Nt` | this library's stride-aware parallel sweep |

Every case cross-checks `libxc-1t` against `rust-Nt` elementwise, and prints an
order-sensitive fingerprint over `to_bits()` of every output, so a codegen
change can be shown bit-exact rather than merely "close".

## The build-flag gap (found 2026-08-17)

libxc's own CMake turns on `ENABLE_XHOST` by default
(`libxc-master/cmake/xhost.cmake`), so the vendored oracle compiles with
`-march=native -O3`. On this box that expands to full AVX-512 (`znver5`).

This repo passed **no** `target-cpu` at all, so every kernel was generated for
baseline `x86-64` — SSE2. Disassembly of the pre-change `gga_x_b88` rlib:

```
5164 mulsd   4610 mulpd   1812 addsd   1474 addpd   584 divsd   539 divpd
```

Not one `ymm`, not one `zmm`, not one VEX-encoded instruction. The comparison
was being run with one side compiled for the actual CPU and the other for a
2003 baseline.

Two separate costs came out of that, and the second is the larger one:

1. **Vector width.** The value-merge pass leaves each output's arithmetic in one
   large basic block, which is what lets LLVM's SLP vectoriser pack independent
   operations — measured at 10–68 % of multiply slots depending on functional.
   At baseline that packing is 2-wide SSE. The hardware does 8-wide.

2. **`f64::mul_add` was a function call.** Without the FMA target feature, LLVM
   lowers `mul_add` to a call into libm's software `fma()`:

   ```asm
   ; -C target-cpu=x86-64          ; -C target-cpu=native
   f: jmpq *fma@GOTPCREL(%rip)     f: vfmadd213sd %xmm2, %xmm1, %xmm0
   ```

   `cbrt_f64` in `kernels-rayon/math` uses two `mul_add`s, and `pow_1_3` is
   called several times per grid point in most functionals (5 in
   `mgga_x_scan` vxc alone). So the hot loop was making ~10 calls per grid
   point into a software FMA that the hardware does in one instruction.

**Enabling `target-cpu=native` does not change any result.** Vectorising
independent scalar operations is exact, and hardware FMA and libm `fma()` are
both correctly-rounded IEEE 754 `fusedMultiplyAdd`, so they agree bit for bit.
The thing that *would* have changed results — LLVM contracting a source-level
`a*b + c` into an FMA — does not happen: rustc leaves `fp-contract` off, which
was checked directly:

```asm
; pub fn g(a,b,c) -> f64 { a * b + c }   with -C target-cpu=native
g: vmulsd %xmm1, %xmm0, %xmm0
   vaddsd %xmm2, %xmm0, %xmm0
```

That is exactly the property `AGENTS.md` requires ("Maple2c formula
translations must preserve floating-point operation order"), so the flag is
safe to turn on. It is verified by fingerprint rather than assumed.

## Where the buffers get zeroed

The kernels accumulate with `+=`, so every output must start at zero. The
generated `prepare()` did that as one `fill(0.0)` per whole output array,
serially, before any arithmetic started. That costs a full extra pass over
every output buffer on one thread, and leaves the memory cold — by the time a
kernel accumulates into a cache line, the zero written into it has been
evicted, so the line is fetched from DRAM twice.

The zeroing now happens per chunk inside `par_sweep`, immediately before the
kernel runs on that chunk (`zero_outputs` in the generated `sweep_*.rs`). The
range is still in L1/L2 when the kernel reads it back, and the clearing is
spread over every worker. Bit-exactness is unaffected: the same value is
stored and the accumulations happen in the same order, only the moment of the
store moves.

## Screening: a correctness bug that was also costing us the tail

`work_gga_inc.c` (and the LDA/MGGA equivalents) open the per-point loop with

```c
dens = ... ;
if(dens < p->dens_threshold)
  continue;
```

so a grid point below threshold costs libxc nothing. The kernels here instead
evaluate the entire formula and select zero at the end through a branch-free
`piecewise3(cond, 0.0, ...)` — `piecewise.rs` documents that both arms are
already evaluated, which is what preserves CubeCL's `select()` semantics and
the operation order.

Chasing that cost turned up something worse than a cost. **Only some kernels
carry a `dens_threshold` guard at all.** The exchange functionals mostly do
(`gga_x_b88`: `let t2 = rho[ip] / 2.0 <= dens_threshold;`). The correlation
functionals mostly do not — `lda_c_vwn`, `gga_c_lyp` and `mgga_c_r2scan` contain
no reference to `dens_threshold` anywhere. libxc gets away with that because its
screen lives *outside* the maple2c body, in `work_*_inc.c`, so it covers every
functional uniformly. This library called the body directly, so for an unguarded
functional the empty tail of a molecular grid received the raw formula value
where libxc returns exactly zero.

Measured against C libxc on a grid with 40 % of its points below threshold:

| functional | in-kernel guard | worst rel. difference vs libxc |
|---|---|---|
| `lda_c_vwn` | none | **1.000** (i.e. 100 %, on `zk`) |
| `gga_c_lyp` | none | **1.000** (on `zk`) |
| `gga_x_b88` | yes | 1.6e-15 |
| `mgga_x_scan` | yes | 6.1e-11 |

Exactly the functionals without the guard disagreed, and exactly the ones with
it agreed — so the cause is not in doubt.

`screened_call` in the generated `sweep_*.rs` now applies libxc's own test
(`total density < dens_threshold`) before the kernel sees a point, which fixes
all of them at once and takes the performance back at the same time. After the
fix `lda_c_vwn` agrees to 3.9e-15 and `gga_c_lyp` to 2.6e-12, while every
already-guarded functional's numbers are unchanged to the last digit — screening
a point whose kernel output was `piecewise3(guard, 0.0, ..)` gives the same
`+0.0` the buffer already held.

It takes one of two routes, chosen from a single forward pass over `rho`:
maximal above-threshold runs get their own kernel call (a real quadrature orders
points by radial shell, so its empty points are contiguous and this costs one
call per chunk, or none for a wholly empty chunk); a chunk fragmented finely
enough that runs would average under 64 points instead runs the kernel over
everything and re-zeros the screened points afterwards. The second route saves
no arithmetic, but it is what stops a pathological grid from turning screening
into a 3.5x regression — splitting into ~1.7-point runs cost about 14 ns per
call and took `gga_x_b88` from 1.98 to 6.84 ns/point before the fallback existed.

## Results

`--np 100000 --reps 9`, ns per grid point, lower is better. "was" is this repo
before any of the changes above. Every `rust` figure carries an identical output
fingerprint before and after, so none of these speedups moved a single bit.

### Chemically active region (no below-threshold points)

| case | libxc-1t | rust-1t was | rust-1t now | vs libxc | libxc-Nt | rust-Nt was | rust-Nt now | **vs libxc** |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| `lda_c_vwn` exc+vxc unpol | 67.6 | 73.4 | 70.72 | 0.97x | 15.21 | 14.83 | 12.34 | **1.11x** |
| `gga_x_b88` exc+vxc unpol | 32.8 | 26.2 | 9.25 | 3.60x | 8.21 | 5.13 | 2.18 | **3.69x** |
| `gga_c_lyp` exc+vxc unpol | 41.4 | 24.2 | 9.43 | 4.37x | 8.14 | 5.41 | 2.24 | **3.50x** |
| `gga_x_b88` exc+vxc+fxc unpol | 57.1 | 35.5 | 12.92 | 4.45x | 12.23 | 7.04 | 3.04 | **3.92x** |
| `gga_x_b88` exc+vxc pol | 107.2 | 73.3 | 25.52 | 4.23x | 27.00 | 14.86 | 5.55 | **4.67x** |
| `mgga_x_scan` exc+vxc unpol | 85.5 | 48.1 | 27.47 | 3.11x | 19.91 | 11.15 | 5.89 | **3.22x** |
| `mgga_c_r2scan` exc+vxc unpol | 174.5 | 124.2 | 78.50 | 2.26x | 40.61 | 24.18 | 14.23 | **2.44x** |
| `mgga_x_scan` exc+vxc pol | 201.9 | 119.6 | 60.99 | 3.36x | 47.42 | 26.59 | 13.31 | **3.41x** |

`lda_c_vwn` is the one case that is only a tie single-threaded. It spends its
time in four `log`s and two `atan`s per point, which both libraries take from
the same libm; there is no arithmetic left to win. Beating it would mean a
vectorised transcendental library, which would not be correctly rounded and so
would break the bit-exactness the rest of this rests on.

### Realistic grid: 40 % of points below threshold, contiguous (`XCVS_TAIL=0.4`)

| case | libxc-1t | rust-1t | vs | libxc-Nt | rust-Nt | **vs** |
|---|--:|--:|--:|--:|--:|--:|
| `lda_c_vwn` exc+vxc unpol | 41.2 | 42.99 | 0.96x | 7.89 | 7.92 | **1.00x** |
| `gga_x_b88` exc+vxc unpol | 20.5 | 6.36 | 3.22x | 5.08 | 1.53 | **3.32x** |
| `gga_c_lyp` exc+vxc unpol | 25.1 | 6.36 | 3.94x | 4.91 | 1.59 | **3.09x** |
| `gga_x_b88` exc+vxc+fxc unpol | 35.0 | 8.65 | 4.05x | 7.52 | 1.92 | **3.92x** |
| `gga_x_b88` exc+vxc pol | 65.2 | 16.97 | 3.84x | 16.36 | 3.76 | **4.35x** |
| `mgga_x_scan` exc+vxc unpol | 52.3 | 17.51 | 2.98x | 10.77 | 3.62 | **2.98x** |
| `mgga_c_r2scan` exc+vxc unpol | 107.3 | 49.16 | 2.18x | 23.84 | 9.43 | **2.53x** |
| `mgga_x_scan` exc+vxc pol | 125.1 | 39.47 | 3.17x | 28.69 | 8.42 | **3.41x** |

Worst case for the screener is a grid that scatters its below-threshold points
individually (`XCVS_TAIL_LAYOUT=scatter`) — no real quadrature does this, but it
is what would expose a regression. There the run-splitting path is abandoned for
compute-then-re-zero, and `rust-Nt` still leads on 7 of 8 cases (1.31x–2.63x);
`lda_c_vwn` is the exception at 0.66x, because libxc's `continue` skips 40 % of
six transcendentals per point and nothing here can.

### Memory

Both libraries allocate **nothing** per evaluation. Measured with a counting
global allocator for the Rust side and `mallinfo2` for the C heap (libxc is
statically linked into the benchmark, so it reaches `malloc` directly and would
otherwise be invisible):

```
one libxc evaluation: 0 rust allocs / 0 B, malloc in-use delta +0 B
one rust  evaluation: 0 rust allocs / 0 B, malloc in-use delta +0 B
```

That holds for every case in the table, LDA through polarized MGGA. Peak RSS
over a whole timed case stays within a few MB of the caller's own buffers, and
the recursive `rayon::join` split allocates nothing: workers get disjoint
`&mut` sub-slices of the caller's arrays, so there is no staging buffer, no
per-chunk scratch and no device-side copy. The typed API still *requires* a
buffer for every output of the requested order; the C-ABI shim accepts
libxc's NULLs (`xc_gga_vxc` without `zk`, `xc_mgga_vxc` without `vlapl`) and
points them at a thread-local stand-in sized to the NULL fields only -- see
"The C-ABI shim's scratch" below for what it did before 2026-09-07.

## Verification

| check | result |
|---|---|
| output fingerprints, before vs after all three changes | identical for all 8 cases |
| `revalcheck` (chunked vs whole-grid, bitwise) | 482,775,350 values identical; 4 differing in `gga_c_op_pw91 Lxc Polarized`, **pre-existing** — reproduced on the untouched tree |
| `crates/kernels-rayon/oracle` vs C libxc, 1e-12 | 337/344 within tolerance; the same 7 fields and the same 3 functionals (`gga_x_fd_lb94`, `gga_x_beefvdw`, `gga_c_hcth_a`) already recorded in `AGENTS.md` |
| screening, elementwise vs C libxc on a 40 % tail grid | `lda_c_vwn` 1.0 -> 3.9e-15, `gga_c_lyp` 1.0 -> 2.6e-12; guarded functionals unchanged to the last digit |

## What was tried and rejected

**Eliminating slice bounds checks.** The generated kernels index `rho[ip]`,
`vrho[ip]` and so on against a trip count taken from a *different* slice
(`for ip in 0..zk.len()`), so LLVM cannot prove the other accesses in range:
`lda_c_vwn` alone carried 55 `panic_bounds_check` sites. Reslicing every
parameter to `np * stride` before the loop removes all of them (verified: 0
sites left in the patched functions). Measured on three kernels it was worth
1–4 % single-threaded — and the parallel column moved by the same amount on the
kernels that were *not* patched, so even that is mostly run-to-run drift. Not
worth a tree-wide emitter change, a full regen and a bitwise gate. The
experiment is reproducible: reslice, rebuild, compare fingerprints (they match).

## HSE06 and PBE0: the composite path (2026-09-07)

Two changes, measured separately because they land in different layers.
Same harness, `--np 100000 --reps 7`, ns per grid point, `libxc-Nt` is
the bar. Every `rust` fingerprint is byte-identical before and after both
changes (`hse06` unpol `bf57a89529581840`, pol `462c5fd0c8ef8b4d`, `wpbeh`
`d67311fbdf2bab7d`), and the composite bench now also asserts that
`rust-1t` and `rust-Nt` agree **bit for bit** -- the chunked mix is checked
exactly, not to a tolerance.

### 1. `gga_x_wpbeh` was scalar, and HSE06 is two of them

HSE06 is `wpbeh(omega=0) - 0.25 * wpbeh(omega=0.11) + PBEc`. Before this,
HSE06 was a dead tie with libxc (105.3 vs 105.9 ns/pt parallel; 611 vs 630
single-threaded) because `gga_x_wpbeh` -- 95% of the cost -- ran the scalar
kernel, and the scalar kernel is *slower* than libxc's C (294 vs 270 ns/pt),
as every unvectorised kernel here is since `cbrt`/`ln`/`exp` moved to rmath's
bit-exact scalar forms.

It was never a SIMD candidate because it calls `xc_erfcx` and `xc_E1_scaled`,
scalar helpers (a Faddeeva table and a Chebyshev series, transcribed from
libxc) that have no vector form, and `simd_qualify.py` skipped any kernel with
a helper. `simd.py` now maps those two to `simd::erfcx` / `simd::e1_scaled`,
which run the *same scalar function on each lane*
(`math/src/simd.rs::lanewise`) -- bit-exact by construction -- and everything
else in the body (8 `sqrt`, 4 `ln`, 3 `exp`, 1 `erf` per point) goes eight
wide. Qualified through the normal ledger gate:

| triple | before | after | ratio |
|---|--:|--:|--:|
| `gga_x_wpbeh exc unpol` | 25.81 | 15.26 | 1.69x |
| `gga_x_wpbeh vxc unpol` | 42.49 | 20.19 | 2.10x |
| `gga_x_wpbeh exc pol` | 86.24 | 64.92 | 1.33x |
| `gga_x_wpbeh vxc pol` | 134.06 | 79.98 | 1.68x |
| `gga_x_wpbeh fxc unpol` | 99.22 | 31.25 | 3.17x |

`kxc`/`lxc` are undecided: the tier-4 build was killed for memory on the
4 MB `lxc_pol` body with other builds running, and a screened hybrid's third
and fourth derivatives are not on any SCF or response hot path. They stay
scalar.

### 2. The mix runs leaf by leaf, out of a pooled scratch

`evaluate_mixed_gga` used to run each auxiliary as a whole-grid sweep into an
`np`-sized scratch and then add `w * scratch` into the caller's output in a
serial pass, per field, per auxiliary, on the calling thread -- after a
serial zeroing of every output. It now splits the grid exactly as `par_sweep`
does (`sweep_gga::par_leaves`) and on each leaf runs every auxiliary into a
leaf-sized buffer leased from the workspace's pool
(`EvaluationWorkspace::leaf_scratch`), folding it into the output while the
leaf is in cache. The arithmetic per element is unchanged (`0`, then
`+= w_k * aux_k` in metadata order), so the bits are the same.

Memory is what this is for. The scratch was `np * components` (and
`EvaluationWorkspace::new` promised the all-orders MGGA superset, 767 doubles
per polarized point, 613 MB at 100k points); it is now bounded by
`workers * leaf * components` and does not grow with the grid, and the
whole-grid scratch is allocated only when an LDA or MGGA composite first
needs it:

| HSE06 `vxc`, 100k points | whole-grid scratch | now (pool) | `EvaluationWorkspace::new` used to hold |
|---|--:|--:|--:|
| unpolarized | 4.00 MB | 0.60 MB | 56 MB |
| polarized | 8.00 MB | 1.20 MB | 614 MB |

Speed: for HSE06 the mix layer was never the bottleneck (46.1 -> 47.1 ns/pt
unpol, 106.5 -> 106.6 pol, with the SIMD `wpbeh` in both) -- the serial passes
were a few percent under the kernel cost. For PBE0, whose two auxiliaries are
both cheap SIMD kernels, the serial passes were a visible share and go away:

| case | libxc-Nt | old mix | **new mix** | vs libxc |
|---|--:|--:|--:|--:|
| `hyb_gga_xc_pbeh` exc+vxc unpol | 15.75 | 9.16 | **8.25** | 2.08x |
| `hyb_gga_xc_pbeh` exc+vxc pol | 39.88 | 16.56 | **14.74** | 3.02x |

### Results

| case | libxc-1t | rust-1t was | rust-1t now | libxc-Nt | rust-Nt was | rust-Nt now | **vs libxc** (was) |
|---|--:|--:|--:|--:|--:|--:|--:|
| `hyb_gga_xc_hse06` exc+vxc unpol | 621.3 | 611.4 | 276.5 | 108.7 | 105.3 | **47.1** | **2.31x** (1.01x) |
| `hyb_gga_xc_hse06` exc+vxc pol | 1428.1 | 1497.0 | 627.2 | 259.3 | 242.2 | **106.6** | **2.43x** (1.08x) |
| `gga_x_wpbeh` exc+vxc unpol | 253.0 | 294.3 | 120.0 | 48.4 | 47.5 | **19.9** | **2.43x** (1.03x) |
| `hyb_gga_xc_pbeh` exc+vxc unpol | 104.2 | -- | 49.7 | 17.1 | -- | **8.25** | **2.08x** |
| `hyb_gga_xc_pbeh` exc+vxc pol | 230.8 | -- | 93.8 | 44.5 | -- | **14.7** | **3.02x** |

PBE's own kernels were already on the allowlist and are untouched; their rows
are here so the two names in the question have their numbers side by side:

| case | libxc-Nt | rust-Nt | vs libxc |
|---|--:|--:|--:|
| `gga_x_pbe` exc+vxc unpol | 3.54 | 3.35 | 1.06x (run-to-run noise on a 3 ns kernel: the earlier run gave 4.65 vs 3.46, 1.34x) |
| `gga_x_pbe` exc+vxc pol | 17.98 | 6.16 | 2.92x |
| `gga_x_pbe` exc+vxc+fxc unpol | 6.33 | 3.85 | 1.64x |
| `gga_c_pbe` exc+vxc unpol | 13.14 | 4.69 | 2.80x |
| `gga_c_pbe` exc+vxc pol | 25.91 | 8.41 | 3.08x |
| `gga_c_pbe` exc+vxc+fxc unpol | 21.37 | 6.93 | 3.08x |

`gga_x_pbe` unpolarized `vxc` is the weakest row and it is at the floor: the
kernel is three `cbrt` and ~40 flops per point, `docs/perf/kernel-codegen.md`
already closed hoisting, bounds checks and scheduling on it, and its whole
body is 3.3 ns/pt across 16 threads. The remaining lever there is the
correctly-rounded `cbrt` itself, which is not one this project will pull.

## HSE06: the helpers (2026-09-07, later the same day)

After the section above, `gga_x_wpbeh vxc` was eight lanes wide everywhere
except its two special functions: `xc_erfcx` and `xc_E1_scaled` ran as
`lanewise` -- eight scalar calls per 8-point step, three helper calls per
point (one `erfcx`, two `E1`) -- and a microbenchmark over the argument range
the kernel produces put them at 16.3 and 22.0 ns per point, i.e. about
60 ns of the 123 ns single-threaded point.

Both now have vector forms in `math/src/simd.rs`, bit-identical to the scalar
helpers by construction rather than by tolerance:

- `erfcx`: `y100 = 400/(4+x)` on eight lanes, the interval index taken per
  lane by the same `(int)` truncation libxc's `switch` uses, the seven
  Chebyshev coefficients of that interval gathered from a 100 x 7 table
  (`erfcx_coef.rs`, generated from the scalar `match` by
  `tools/translate_rayon/gen_erfcx_coef.py` so both parse the same
  constants), then the same Horner polynomial. The `x > 50` and `x > 5e7`
  continued-fraction arms are evaluated in vector form and selected per
  lane -- a low-density tail point does reach `x > 50`.
- `e1_scaled`: Clenshaw on eight lanes in the scalar's operand order
  (`(twox*b1 - b2) + cs[i]`), one arm per positive interval (`<= 1`,
  `<= 4`, `> 4`), each evaluated only when some lane needs it and selected
  per lane; `exp`/`ln` are the module's bit-exact forms, which the scalar
  helper also calls.

Vector mul/add/sub/div are IEEE per lane and rustc does not contract them
into FMAs, so an expression with the scalar's grouping gives the scalar's
bits. A lane that is negative or NaN -- which no kernel produces from a
finite input, the arguments being a `sqrt` and sums of squares -- sends the
whole vector down the old lane-wise path. `math/tests/simd_exact.rs` sweeps
every branch of both functions (250k inputs each, dense across the interval
edges, log-spaced to 1e300, shuffled so vectors straddle arms) and requires
`to_bits()` equality on every lane.

| helper, ns per point | scalar (lane-wise) | vector | ratio |
|---|--:|--:|--:|
| `erfcx` | 16.26 | 4.29 | 3.8x |
| `e1_scaled` | 21.95 | 10.28 | 2.1x |

`--np 100000 --reps 7`, ns per grid point; "was" is the previous section's
tree. Every `rust` fingerprint is byte-identical before and after (`hse06`
unpol `bf57a89529581840`, pol `462c5fd0c8ef8b4d`, `wpbeh`
`d67311fbdf2bab7d`), and `rust-1t == rust-Nt` bitwise on the composites:

| case | libxc-1t | rust-1t was | rust-1t now | libxc-Nt | rust-Nt was | rust-Nt now | **vs libxc** (was) |
|---|--:|--:|--:|--:|--:|--:|--:|
| `hyb_gga_xc_hse06` exc+vxc unpol | 618.1 | 279.3 | **198.1** | 100.1 | 41.7 | **30.7** | **3.26x** (2.25x) |
| `hyb_gga_xc_hse06` exc+vxc pol | 1440.2 | 642.4 | **460.0** | 254.0 | 132.5 | **83.0** | **3.06x** (1.92x) |
| `gga_x_wpbeh` exc+vxc unpol | 254.6 | 123.5 | **82.0** | 50.1 | 24.4 | **15.7** | **3.19x** (2.05x) |
| `hyb_gga_xc_pbeh` exc+vxc unpol | 108.9 | 51.8 | 51.2 | 17.7 | 10.0 | 9.0 | 1.97x (unchanged) |
| `hyb_gga_xc_pbeh` exc+vxc pol | 231.8 | 94.5 | 95.3 | 51.5 | 18.4 | 19.0 | 2.71x (unchanged) |

("was" ratios are recomputed against this run's `libxc-Nt`; the previous
section quoted 2.31x/2.43x against its own.) The `wpbeh` and `pbeh` rows
are the mean of two runs each. PBE0 does not call either helper and moves
only by noise. What is left in `wpbeh` is the bit-exact `ln`/`exp`/`sqrt`
and the erf, and `e1_scaled` at 10 ns: when a vector's lanes straddle the
`x <= 1` / `x > 1` boundary both arms run, and a coherent grid keeps that
rarer than the microbenchmark's log-uniform draw does.

## The C-ABI shim's scratch (2026-09-07)

Not a benchmark result -- the benchmark calls the typed API -- but the path
a C or Fortran DFT code takes to HSE06 or PBE, and the largest memory defect
in the tree. Every `xc_lda_*` and `xc_gga_*` entry point allocated and
zero-filled

```rust
let mut scratch = vec![0.0; dims.total_output_components() * np];
```

on every call: every output of every derivative order through `lxc`, 15
doubles per unpolarized GGA point and 126 per polarized one, whether or not
any pointer was NULL. At a million polarized points that is a **1.0 GB
allocation and memset per `xc_gga_exc_vxc` call**, against 48 MB of
results. The MGGA entry points had the opposite defect: no stand-in at all,
so `xc_mgga_vxc` with a NULL `zk`, or a NULL `vlapl` from a caller whose
functional does not use the laplacian (which is how every tau-only MGGA is
called), failed with an output-buffer-size error.

Both now go through one thread-local `FillBuf` (`legacy_eval.rs`), sized to
the NULL-but-required fields only and reused across calls. A call with every
pointer supplied touches no scratch; `xc_gga_vxc` with a NULL `zk` uses `np`
doubles, allocated once per thread and size. `legacy_eval.rs` tests it with a
counting allocator: after warm-up, neither form allocates a byte, and the
NULL-`zk` and NULL-`vlapl` forms agree bit for bit with the all-pointers
call. The workspace was already thread-local and reused; this closes the
other allocation on that path.
