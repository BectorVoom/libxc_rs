# Archived: CubeCL kernel translator

This is the translator that emitted the `#[cube]` / CubeCL kernel tree
(253,961 files under `crates/kernels/`). It is **retired**, kept for reference
and for regenerating the historical tree if a comparison is ever needed.

Superseded by `tools/translate_rayon/`, which emits plain-Rust kernels driven by
rayon. See `docs/adr/0001-rayon-over-cubecl.md` for the decision and the
measurements behind it.

## What is here

| path | role |
|------|------|
| `translate_v2/` | the emitter proper: `per_functional.py` (Maple -> kernel bodies), `emit.py` (crate/file layout), `cse.py` (common-subexpression partitioning), `helpers_allowlist.py` |
| `maple_to_kernels.py` | top-level driver, incremental-sync + `--changed-only` manifest |
| `translate_gga.py`, `translate_lda_v2.py`, `translate_mgga.py` | per-family drivers |

## Running it after the move

These modules import each other as `translate_v2.*`, so the archive directory has
to be on `sys.path`:

```bash
PYTHONPATH=tools/archive/cubecl python3 tools/archive/cubecl/maple_to_kernels.py --help
```

`tools/split_per_functional_subcrate.py` also imports `translate_v2` and was left
in place; it needs the same `PYTHONPATH` if used against the archived emitter.

## Why it was retired

Measured on `gga_x_pbe` vxc unpolarized, f64, 10^6 grid points, on a Ryzen AI 7
350 (Zen 5, AVX-512), both legs multi-threaded and buffers already resident:

| leg | best | vs rayon |
|-----|------|----------|
| native rayon | 3.15 ms | 1.00x |
| cubecl-cpu (launch + sync only) | 6.55 ms | 0.48x |
| cubecl-cpu (full call) | 15.36 ms | 0.20x |

The CubeCL path also carried a large build cost: one MGGA functional crate
(`libxc-kernel-mgga_x_tpss`) took >12 min and 1.5 GB RSS to compile, and
`libxc-kernel-mgga_c_tpssloc` cannot be compiled at all on a 30 GB machine
(~25 GB peak in `cubecl-macros` expansion; it is excluded from
`default-members` for that reason).

### Caveats on that decision, recorded honestly

- The benchmark covers **one** GGA first-derivative kernel. The MGGA high-order
  case (`mgga_x_tpss` kxc polarized, ~5,700 live temporaries) was never
  successfully benchmarked -- its kernel crate would not finish compiling.
- Retiring CubeCL retires the **GPU path**. `CLAUDE.md` lists single-source
  CPU+GPU execution as core project value; that capability is given up here.
- Neither leg was vectorised. The emitted loop is division-throughput bound
  (17 scalar `divsd` per grid point, zero `divpd`); LLVM declines to widen it
  on register pressure. So the rayon win is thread scheduling and call
  overhead, not SIMD.
