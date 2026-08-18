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
per-chunk scratch and no device-side copy. Where the two differ is that this
library still *requires* the caller to pass a buffer for every output of the
requested order, while libxc offers `xc_gga_vxc`-style entry points that let
you skip `zk`; closing that would save `np * (1 + nspin + 3)` doubles on a
response calculation that only wants the second derivatives.

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
