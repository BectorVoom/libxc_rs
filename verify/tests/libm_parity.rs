//! `rmath`'s bit-exact surface against the platform libm libxc itself calls.
//!
//! `crates/kernels-rayon/math/tests/simd_exact.rs` already pins every rmath
//! free function this tree uses against `f64::` -- which is glibc on this
//! platform -- and that test is what caught the whole tree running rmath's
//! *Fast* path against a 1e-12 contract. It cannot cover `erf` and `erfc`,
//! because Rust's `f64` has neither, and those two are not incidental: every
//! range-separated functional goes through `erf` (`gga_x_wpbeh`'s `Ga(s)`,
//! `gga_x_ityh`, `lda_x_erf`, the whole `lc_*`/`cam_*`/`hjs_*` family).
//!
//! This crate links libxc, and therefore libm, so the comparison can be made
//! against the same `erf` the C oracle calls.

// Pull `libxc-sys` in so this target links the same libm the C oracle does.
use libxc_sys as _;

unsafe extern "C" {
    fn erf(x: f64) -> f64;
    fn erfc(x: f64) -> f64;
}

/// xorshift, so the sweep is reproducible.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn uniform(&mut self, lo: f64, hi: f64) -> f64 {
        let u = (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        lo + (hi - lo) * u
    }
    /// A magnitude drawn log-uniformly, with a random sign.
    fn logmag(&mut self, lo: f64, hi: f64) -> f64 {
        let e = self.uniform(lo.log10(), hi.log10());
        let m = 10f64.powf(e);
        if self.next_u64() & 1 == 0 { m } else { -m }
    }
}

fn sweep(name: &str, ours: fn(f64) -> f64, theirs: unsafe extern "C" fn(f64) -> f64) {
    let mut rng = Rng(0x243F6A8885A308D3);
    let mut vals: Vec<f64> = vec![
        0.0, -0.0, 1.0, -1.0, 0.5, 1e-30, -1e-30, 2.0, -2.0, 5.0, -5.0,
        6.0, 27.0, -27.0, 1e3, -1e3,
    ];
    for _ in 0..200_000 {
        vals.push(rng.logmag(1e-30, 1e3));
    }
    for _ in 0..200_000 {
        vals.push(rng.uniform(-6.0, 6.0));
    }

    let mut mism = 0usize;
    let mut worst = (0.0f64, 0.0f64, 0.0f64);
    for &x in &vals {
        let a = ours(x);
        let b = unsafe { theirs(x) };
        if a.to_bits() != b.to_bits() {
            mism += 1;
            let ulps = (a.to_bits() as i64 - b.to_bits() as i64).unsigned_abs() as f64;
            if ulps > worst.2 {
                worst = (x, a - b, ulps);
            }
        }
    }
    println!(
        "{name}: {mism} of {} differ from libm; worst at x = {:e} (delta {:e}, {} ulp)",
        vals.len(),
        worst.0,
        worst.1,
        worst.2
    );
    assert_eq!(
        mism, 0,
        "{name} is not bit-exact against the libm libxc calls: {mism} of {} inputs differ, \
         worst at x = {:e} by {} ulp. Every transcendental in this tree has to be the \
         BitExact rmath form (see crates/kernels-rayon/math/src/rmath_bitexact.rs); a \
         mismatch here means a call site reached the Fast path, or rmath's kernel drifted.",
        vals.len(),
        worst.0,
        worst.2
    );
}

#[test]
fn rmath_erf_is_bit_exact_against_libm() {
    sweep("erf", libxc_rkernel_math::rmath::erf, erf);
}

#[test]
fn rmath_erfc_is_bit_exact_against_libm() {
    sweep("erfc", libxc_rkernel_math::rmath::erfc, erfc);
}
