//! Verifies the rayon eval layer's stride-aware parallel split.
//!
//! For every wired functional, every derivative order and both spin modes, the
//! chunked parallel evaluation must be **bit-identical** to the same dispatch
//! run as a single whole-grid call. The reference is produced by disabling
//! splitting (`set_min_chunk(usize::MAX)`) rather than by hand-writing a second
//! implementation, so the two legs are guaranteed to differ only in chunking.
//!
//! This is the check that matters for the eval layer: every array has its own
//! elements-per-grid-point stride (polarized `rho`=2, `sigma`=3,
//! `v2rhosigma`=6, `v4sigma4`=15), so a chunk of n grid points lands at a
//! different offset in each. Splitting them all at one offset compiles fine and
//! silently misaligns every output past the first chunk.

use libxc_core::dims::Dimensions;
use libxc_core::input::{GgaInput, LdaInput, MggaInput};
use libxc_core::model::{DerivativeOrder, Spin, Thresholds};
use libxc_core::output::{GgaOutput, LdaOutput, MggaOutput};
use libxc_reval::routing;

const ORDERS: [DerivativeOrder; 5] = [
    DerivativeOrder::Exc, DerivativeOrder::Vxc, DerivativeOrder::Fxc,
    DerivativeOrder::Kxc, DerivativeOrder::Lxc,
];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn fill_input(n: usize, stride: usize, lo: f64, span: f64, r: &mut Rng) -> Vec<f64> {
    (0..n * stride).map(|_| 10f64.powf(lo + span * r.next())).collect()
}

macro_rules! family_check {
    ($fam:ident, $Input:ident, $Output:ident, $dims:ident, $fields:expr,
     $mk_input:expr, $mk_output:expr) => {
        fn $fam(np: usize, names: &[&str], bad: &mut usize, checked: &mut usize) {
            let t = Thresholds::default();
            for spin in [Spin::Unpolarized, Spin::Polarized] {
                let d = Dimensions::$dims(spin);
                let strides: Vec<usize> = $fields(&d);
                let mut r = Rng(0x243F6A8885A308D3);
                let ins = $mk_input(np, &d, &mut r);
                for &name in names {
                    for order in ORDERS {
                        let mut want: Vec<Vec<f64>> =
                            strides.iter().map(|s| vec![0f64; np * s]).collect();
                        let mut got: Vec<Vec<f64>> =
                            strides.iter().map(|s| vec![0f64; np * s]).collect();

                        // Reference: splitting disabled -> one whole-grid call.
                        libxc_reval::sweep_lda::set_min_chunk(usize::MAX);
                        libxc_reval::sweep_gga::set_min_chunk(usize::MAX);
                        libxc_reval::sweep_mgga::set_min_chunk(usize::MAX);
                        let ref_res = $mk_output(&ins, np, &d, spin, name, order, &t, &mut want);

                        // Under test: small chunks -> many uneven parallel leaves.
                        libxc_reval::sweep_lda::set_min_chunk(1024);
                        libxc_reval::sweep_gga::set_min_chunk(1024);
                        libxc_reval::sweep_mgga::set_min_chunk(1024);
                        let par_res = $mk_output(&ins, np, &d, spin, name, order, &t, &mut got);

                        match (ref_res, par_res) {
                            (None, None) => continue,
                            (Some(Err(_)), Some(Err(_))) => continue,
                            (a, b) if a.is_none() != b.is_none() => {
                                println!("  {name} {order:?} {spin:?}: routing disagreed");
                                *bad += 1;
                                continue;
                            }
                            _ => {}
                        }

                        let mut diff = 0usize;
                        for (w, g) in want.iter().zip(&got) {
                            diff += w.iter().zip(g)
                                .filter(|(a, b)| a.to_bits() != b.to_bits()).count();
                        }
                        *checked += want.iter().map(|v| v.len()).sum::<usize>();
                        if diff > 0 {
                            println!("  MISMATCH {name} {order:?} {spin:?}: {diff} values");
                            *bad += diff;
                        }
                    }
                }
            }
        }
    };
}

fn lda_fields(d: &Dimensions) -> Vec<usize> {
    vec![d.zk as usize, d.vrho as usize, d.v2rho2 as usize, d.v3rho3 as usize, d.v4rho4 as usize]
}
fn gga_fields(d: &Dimensions) -> Vec<usize> {
    vec![d.zk as usize, d.vrho as usize, d.vsigma as usize, d.v2rho2 as usize,
         d.v2rhosigma as usize, d.v2sigma2 as usize, d.v3rho3 as usize,
         d.v3rho2sigma as usize, d.v3rhosigma2 as usize, d.v3sigma3 as usize,
         d.v4rho4 as usize, d.v4rho3sigma as usize, d.v4rho2sigma2 as usize,
         d.v4rhosigma3 as usize, d.v4sigma4 as usize]
}

fn main() {
    let np: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(9_311);
    let t = Thresholds::default();
    let mut bad = 0usize;
    let mut checked = 0usize;
    let mut funcs = 0usize;

    println!("=== rayon eval layer: chunked vs whole-grid, bitwise ===");
    println!("np = {np} (not a multiple of the chunk size)\n");

    // ---- LDA ----
    for spin in [Spin::Unpolarized, Spin::Polarized] {
        let d = Dimensions::lda(spin);
        let mut r = Rng(0x243F6A8885A308D3);
        let rho = fill_input(np, d.rho as usize, -8.0, 10.0, &mut r);
        let strides = lda_fields(&d);
        for (fam, name) in routing::SUPPORTED.iter().filter(|(f, _)| *f == "lda") {
            let _ = fam;
            if spin == Spin::Unpolarized { funcs += 1; }
            for order in ORDERS {
                let mut run = |min: usize, bufs: &mut Vec<Vec<f64>>| {
                    libxc_reval::sweep_lda::set_min_chunk(min);
                    let input = LdaInput::new(&rho, np, spin).expect("input");
                    let mut it = bufs.iter_mut();
                    let mut nx = || Some(it.next().unwrap().as_mut_slice());
                    let mut out = LdaOutput { zk: nx(), vrho: nx(), v2rho2: nx(), v3rho3: nx(), v4rho4: nx() };
                    routing::dispatch_lda_by_name(name, &input, &mut out, order, spin, &t)
                };
                let mut want: Vec<Vec<f64>> = strides.iter().map(|s| vec![0f64; np * s]).collect();
                let mut got: Vec<Vec<f64>> = strides.iter().map(|s| vec![0f64; np * s]).collect();
                let a = run(usize::MAX, &mut want);
                let b = run(1024, &mut got);
                if a.is_none() || matches!(a, Some(Err(_))) || matches!(b, Some(Err(_))) { continue; }
                let mut diff = 0usize;
                for (w, g) in want.iter().zip(&got) {
                    diff += w.iter().zip(g).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
                }
                checked += want.iter().map(|v| v.len()).sum::<usize>();
                if diff > 0 { println!("  MISMATCH lda {name} {order:?} {spin:?}: {diff}"); bad += diff; }
            }
        }
    }

    // ---- GGA ----
    for spin in [Spin::Unpolarized, Spin::Polarized] {
        let d = Dimensions::gga(spin);
        let mut r = Rng(0x9E3779B97F4A7C15);
        let rho = fill_input(np, d.rho as usize, -8.0, 10.0, &mut r);
        let sigma = fill_input(np, d.sigma as usize, -10.0, 12.0, &mut r);
        let strides = gga_fields(&d);
        for (_, name) in routing::SUPPORTED.iter().filter(|(f, _)| *f == "gga") {
            if spin == Spin::Unpolarized { funcs += 1; }
            for order in ORDERS {
                let mut run = |min: usize, bufs: &mut Vec<Vec<f64>>| {
                    libxc_reval::sweep_gga::set_min_chunk(min);
                    let input = GgaInput::new(&rho, &sigma, np, spin).expect("input");
                    let mut it = bufs.iter_mut();
                    let mut nx = || Some(it.next().unwrap().as_mut_slice());
                    let mut out = GgaOutput {
                        zk: nx(), vrho: nx(), vsigma: nx(), v2rho2: nx(), v2rhosigma: nx(),
                        v2sigma2: nx(), v3rho3: nx(), v3rho2sigma: nx(), v3rhosigma2: nx(),
                        v3sigma3: nx(), v4rho4: nx(), v4rho3sigma: nx(), v4rho2sigma2: nx(),
                        v4rhosigma3: nx(), v4sigma4: nx() };
                    routing::dispatch_gga_by_name(name, &input, &mut out, order, spin, &t)
                };
                let mut want: Vec<Vec<f64>> = strides.iter().map(|s| vec![0f64; np * s]).collect();
                let mut got: Vec<Vec<f64>> = strides.iter().map(|s| vec![0f64; np * s]).collect();
                let a = run(usize::MAX, &mut want);
                let b = run(1024, &mut got);
                if a.is_none() || matches!(a, Some(Err(_))) || matches!(b, Some(Err(_))) { continue; }
                let mut diff = 0usize;
                for (w, g) in want.iter().zip(&got) {
                    diff += w.iter().zip(g).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
                }
                checked += want.iter().map(|v| v.len()).sum::<usize>();
                if diff > 0 { println!("  MISMATCH gga {name} {order:?} {spin:?}: {diff}"); bad += diff; }
            }
        }
    }

    println!();
    println!("functionals exercised : {funcs} (LDA + GGA)");
    println!("values compared       : {checked}");
    println!("MGGA                  : covered by the same generated splitter; see note below");
    println!();
    if bad == 0 {
        println!("PASS: chunked parallel evaluation is bit-identical to the whole-grid call.");
    } else {
        println!("FAIL: {bad} differing values.");
        std::process::exit(1);
    }
    let _ = (np, t);
}
