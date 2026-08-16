# ADR 0001: Replace the CubeCL kernel substrate with plain Rust + rayon

Status: **accepted** — decided by the project owner after the measurements below.

## Context

The kernel tree was authored as CubeCL `#[cube]` kernels so that one source
could target CPU and GPU. That is the "core value" stated in `CLAUDE.md`.
Benchmarking the CPU path put the premise in doubt.

Measured on `gga_x_pbe` vxc unpolarized, f64, 10^6 grid points, Ryzen AI 7 350
(Zen 5, AVX-512), both legs multi-threaded, buffers already device-resident,
best of ~290 contention-free reps (reps where foreign processes stole CPU were
discarded, not averaged in):

| leg | best | vs rayon |
|-----|------|----------|
| native rayon | 3.15 ms | 1.00x |
| cubecl-cpu (launch + sync only) | 6.55 ms | 0.48x |
| cubecl-cpu (full call) | 15.36 ms | 0.20x |
| cubecl-hip (launch + sync only) | 10.47 ms | 0.30x |

Build cost was the second factor. Under CubeCL, one MGGA functional crate
(`libxc-kernel-mgga_x_tpss`) took >12 min and 1.5 GB RSS to compile, and
`libxc-kernel-mgga_c_tpssloc` could not be compiled at all on a 30 GB machine
(~25 GB peak in `cubecl-macros` expansion; excluded from `default-members` for
that reason). The `partN` / `chunkK` / `metaM` fan-out in the generated tree
exists solely to get under that ceiling.

## Decision

Retire the CubeCL substrate. Kernels become plain Rust functions over `&[f64]`
slices; parallelism is rayon, applied by the caller.

## Consequences

**Gained.** ~2-3x on the CPU hot path. Build cost collapses: the same
`mgga_c_tpssloc` that could not compile now checks in 26.8 s at 1.47 GB, and
`mgga_c_revtpss_p4` (9,612 files, the deepest crate) in 4.9 s at 1.06 GB.
Elementary functions (`ln`, `sqrt`, `exp`, `powf`) now resolve to the system
libm -- the same libm libxc calls -- instead of per-backend intrinsics, so the
backend sits *closer* to the oracle. The generated tree contains no `unsafe`.

**Given up.** The GPU path, entirely. `CLAUDE.md` still lists single-source
CPU+GPU execution as core value and needs updating. The CubeCL HIP leg was also
the only evidence about GPU behaviour, and it was already losing to the CPU on
this hardware (10.47 ms vs 3.15 ms) -- but that is one consumer iGPU with
1/32-rate f64, not a datacenter card.

### What the decision is NOT based on

Recorded so this is not later mistaken for a broader result than it is:

- **One kernel.** `gga_x_pbe` vxc unpolarized, first derivatives. The MGGA
  high-order case (`mgga_x_tpss` kxc polarized, ~5,700 live temporaries) was
  never successfully benchmarked -- its CubeCL crate would not finish compiling,
  which is itself part of the argument, but it means the runtime comparison for
  the heaviest kernels is unmeasured.
- **Neither leg was vectorised.** The emitted loop is division-throughput bound:
  17 scalar `divsd` per grid point, zero `divpd`. LLVM declines to widen it on
  register pressure even when the source is explicitly blocked. So the rayon win
  is thread scheduling and call overhead, not SIMD, and the ~3-4x SIMD ceiling
  is unclaimed by either backend.
- **An early measurement said the opposite** (cubecl-cpu at 0.79x, then 1.26x of
  rayon). That was a harness error: the native legs were compiled without
  `-C target-cpu=native`, so they ran as baseline x86-64 (SSE2, `mul_add`
  lowering to a libm `fma()` call) while cubecl-cpu JIT-compiled with native
  features. Corrected, rayon leads.

## Migration mechanism

`tools/translate_rayon/` transforms the existing CubeCL tree rather than
re-deriving kernels from the Maple sources. That is deliberate: the transform is
mechanical and preserves floating-point operation order exactly, so it cannot
introduce a translation bug the CubeCL tree does not already have, and the
result can be checked against the old tree by *bit* comparison rather than by
tolerance. Re-deriving from Maple would have put all 649 functionals back at
risk of fresh translation errors.

The old emitter is archived under `tools/archive/cubecl/`, and the generated
CubeCL kernel tree under `archive/kernels-cubecl/` (formerly `crates/kernels/`).

Both remain buildable on purpose. The bit-exactness gate compares the rayon
kernels against the CubeCL ones, and only one functional has been through it so
far, so the archived tree is still load-bearing for verification. It is a
workspace member but not in `default-members`, so a bare `cargo build` no longer
compiles 305 CubeCL crates.

## Status of the migration

Done and verified:

- **254,180 kernel bodies translated**, exactly 1:1 with the CubeCL tree.
- **All 305 functional crates compile**, 0 failures (full `cargo check` sweep).
- **All 15 math modules ported**; math crate compiles clean.
- **Kernel bit-exactness** (`crates/kernels-rayon/verify`, `rkverify`):
  `gga_x_pbe` vxc unpolarized is bit-identical to the CubeCL kernel over
  300,000 points including zero, threshold-crossing, subnormal and 1e300
  inputs, and the chunked rayon sweep matches the serial one exactly.
- **Eval-layer correctness** (`crates/libxc-reval`, `revalcheck`): all ten
  (order, spin) arms bit-identical to a direct whole-grid call. Polarized Lxc
  is 6,455,862 values per arm across strides up to 15, at a grid size that is
  deliberately not a multiple of the chunk size so uneven leaves are exercised.
- **Throughput**: the rayon eval layer runs `gga_x_pbe` vxc unpolarized at
  n = 1e6 in ~7.4 ms, against 15.36 ms for the CubeCL full call (~11 ms after
  the buffer-management fix) -- about 2.1x. The residual gap to the 3.15 ms
  raw kernel is mostly the mandatory zeroing of caller buffers, which the `+=`
  accumulation contract requires.

Not done:

- `libxc-reval` covers **GGA only, and routes one functional** (`gga_x_pbe`)
  directly; there is no `GgaFunctional` match arm yet. The remaining 304 GGA
  functionals are one `funcs/*.rs` each and port from the old dispatch files
  with a crate-path change.
- **LDA and MGGA eval layers**: same pattern, but `sweep.rs` needs an
  `LdaChunk` and an `MggaChunk` (MGGA has 4 inputs and ~70 outputs).
- The old CubeCL eval path (`crates/libxc-eval`) is untouched and still builds
  (repointed at `archive/kernels-cubecl/`), so the two can be compared until the
  migration completes.
- **Only `gga_x_pbe` vxc unpolarized has been bit-verified against CubeCL.** The
  other 304 functionals compile but have not been through the gate. Everything
  here also chains back to the CubeCL tree being correct — the transform
  guarantees bit-equality with *it*, not with libxc. The oracle suite has not
  been run against the rayon tree.
- `CLAUDE.md` still lists single-source CPU+GPU execution as core project
  value. That is no longer true and should be rewritten.

### Translator gaps found by the full-tree sweep

The single-functional pilot could not have caught these; each was fixed and
re-verified:

1. The emitter walked one directory level, but the tree is four deep
   (`src/<output>/partN/metaM/chunkK.rs`). It wrote `mod.rs` files referencing
   modules that were never emitted.
2. The `::<f64>` turbofish strip ran before `F` -> `f64`, so every call inside
   a generic helper had its turbofish recreated after the strip.
3. Sibling-crate references (`mgga_c_tpssloc_p0`..`_p6`) were not renamed and
   their dependencies not emitted; a synthesised `lib.rs` also dropped the
   `pub use` re-exports those siblings are imported through.
4. `F::cast_from(x)` was stripped unconditionally, but it is the identity only
   when `x` is already f64 -- in `bspline` it wrapped a `u32`, so stripping it
   silently dropped a real conversion. **This one produced wrong numbers, not a
   compile error.** It now strips float literals and emits `(x as f64)` otherwise.
5. `#[derive(CubeType)]` on the chunk-first struct interface (`gga_c_pbe`,
   `mgga_x_pbe_gx`), and struct *uses* `Chunk0Out<F>` keeping a generic after
   the *declaration* lost it.
