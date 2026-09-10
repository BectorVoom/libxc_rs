//! The three iterative helpers, against libxc's own C.
//!
//! `mgga_x_br89`'s exchange-hole inversion, `mgga_x_mbrxc_bg`'s cuspless-hole
//! inversion and `LambertW` are the only places in this tree where the answer
//! depends on a *stopping rule* rather than on an expression. Everything else
//! is a straight-line formula whose fidelity a fingerprint or a functional
//! comparison can pin down; these are loops, and a loop that runs a different
//! number of times lands somewhere else inside the same tolerance.
//!
//! They used to be CubeCL-era transcriptions -- 60 (resp. 15) unconditionally
//! unrolled steps with branchless `select`, because `#[cube]` kernels had no
//! dynamic loops -- and Brent does not stand still once converged. Comparing
//! whole functionals could only ever say "`mgga_x_br89`'s `vsigma` is 3.8e-9
//! out"; comparing the helper says which digit of which iterate.
//!
//! All three are ordinary exported symbols in the static library
//! (`xc_mgga_x_br89_get_x` and `xc_mgga_x_mbrxc_get_x` are declared in
//! `util.h`, `LambertW` in `special_functions.c`), so they can be called
//! directly rather than through a functional that happens to use them.

// Pull `libxc-sys` in so its `cargo:rustc-link-lib=static=xc` reaches this
// target; nothing here goes through the generated bindings, but the symbols
// below live in that archive.
use libxc_sys as _;

unsafe extern "C" {
    fn xc_mgga_x_br89_get_x(q: f64) -> f64;
    fn xc_mgga_x_mbrxc_get_x(q: f64) -> f64;
    #[allow(non_snake_case)]
    fn LambertW(z: f64) -> f64;
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
    /// A magnitude drawn log-uniformly, optionally signed.
    fn logmag(&mut self, lo: f64, hi: f64, signed: bool) -> f64 {
        let e = self.uniform(lo.log10(), hi.log10());
        let m = 10f64.powf(e);
        if signed && self.next_u64() & 1 == 0 { -m } else { m }
    }
}

/// Is the oracle this run links built with `-ffp-contract=off`?
///
/// The same environment variable `libxc-sys/build.rs` reads. When it is set,
/// the C evaluates `a*b + c` as written and every helper below is *bit*
/// identical; when it is not, GCC contracts and the two implementations differ
/// by the amount the iteration's own tolerance leaves open.
fn contraction_off() -> bool {
    std::env::var("LIBXC_RS_FP_CONTRACT").as_deref() == Ok("off")
}

/// The result of one sweep.
struct Diff {
    n: usize,
    total: usize,
    worst_abs: f64,
    worst_rel: f64,
    at: f64,
    ours: f64,
    theirs: f64,
}

fn sweep(
    vals: &[f64],
    ours: impl Fn(f64) -> f64,
    theirs: unsafe extern "C" fn(f64) -> f64,
    skip: impl Fn(f64) -> bool,
) -> Diff {
    let mut d = Diff {
        n: 0, total: vals.len(), worst_abs: 0.0, worst_rel: 0.0,
        at: 0.0, ours: 0.0, theirs: 0.0,
    };
    for &x in vals {
        if skip(x) {
            d.total -= 1;
            continue;
        }
        let a = ours(x);
        let b = unsafe { theirs(x) };
        if a.to_bits() == b.to_bits() {
            continue;
        }
        d.n += 1;
        let abs = (a - b).abs();
        if abs > d.worst_abs {
            d.worst_abs = abs;
            d.worst_rel = if b == 0.0 { abs } else { abs / b.abs() };
            d.at = x;
            d.ours = a;
            d.theirs = b;
        }
    }
    d
}

/// Assert against whichever oracle is linked.
///
/// With contraction off the claim is absolute: the transcription is libxc's
/// algorithm, operand for operand, so not one input may differ. With
/// contraction on the honest claim is that the two land inside the iteration's
/// own tolerance of each other -- `bound` is that tolerance, and it is the
/// solver's, not a number picked to make the test pass.
fn check(name: &str, d: &Diff, bound: f64, what: &str) {
    println!(
        "{name}: {} of {} differ; worst |delta| {:.3e} (rel {:.3e}) at x = {:.17e} \
         (ours {:.17e}, libxc {:.17e})",
        d.n, d.total, d.worst_abs, d.worst_rel, d.at, d.ours, d.theirs
    );
    if contraction_off() {
        assert_eq!(
            d.n, 0,
            "{name} is not bit-identical to libxc built with -ffp-contract=off: {} of {} \
             inputs differ, worst |delta| {:.3e} at x = {:.17e} (ours {:.17e}, libxc \
             {:.17e}). With contraction off the C evaluates the same expressions in the \
             same order this does, so any difference at all is a transcription bug -- a \
             stopping rule, an operand order, or a constant. See \
             crates/kernels-rayon/math/src/brent.rs.",
            d.n, d.total, d.worst_abs, d.at, d.ours, d.theirs
        );
        return;
    }
    assert!(
        d.worst_abs <= bound,
        "{name} differs from libxc by {:.3e} at x = {:.17e} (ours {:.17e}, libxc {:.17e}), \
         beyond {what} ({bound:.3e}). Against a stock GCC oracle a difference up to that \
         bound is GCC's FMA contraction -- proven by re-running with \
         LIBXC_RS_FP_CONTRACT=off, where all three of these are bit-identical -- but \
         anything larger is not.",
        d.worst_abs, d.at, d.ours, d.theirs
    );
}

/// The `Q` range `mgga_x_br89` actually probes, plus the short-circuit band.
#[test]
fn br89_inversion_is_bit_identical_to_libxc() {
    let mut rng = Rng(0x243F6A8885A308D3);
    let mut vals = vec![
        0.0, 5e-12, -5e-12, 4.9e-12, -4.9e-12, 1e-11, -1e-11, 1.0, -1.0, 1e3, -1e3,
    ];
    for _ in 0..200_000 {
        vals.push(rng.logmag(1e-11, 1e8, true));
    }
    for _ in 0..50_000 {
        vals.push(rng.uniform(-50.0, 50.0));
    }
    let d = sweep(&vals, libxc_rkernel_math::br89::xc_mgga_x_br89_get_x, xc_mgga_x_br89_get_x, |_| false);
    // libxc's `TOL` for this inversion is 5e-12 on the *bracket*, and both
    // sides return its midpoint, so two runs of the same algorithm on
    // ulp-different function values can land anywhere inside it. At a root of
    // 3e-8 that is 1e-4 in relative terms, which is why the relative number
    // looks alarming and the absolute one does not.
    check("xc_mgga_x_br89_get_x", &d, 5e-12, "the solver's own bracket tolerance");
}

#[test]
fn mbrxc_inversion_is_bit_identical_to_libxc() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut vals = vec![0.0, 5e-12, -5e-12, 4.9e-12, 1.0, -1.0, 1e3, -1e3];
    for _ in 0..200_000 {
        vals.push(rng.logmag(1e-11, 1e8, true));
    }
    for _ in 0..50_000 {
        vals.push(rng.uniform(-50.0, 50.0));
    }
    let d = sweep(&vals, libxc_rkernel_math::mbrxc::xc_mgga_x_mbrxc_get_x, xc_mgga_x_mbrxc_get_x, |_| false);
    check("xc_mgga_x_mbrxc_get_x", &d, 5e-12, "the solver's own bracket tolerance");
}

/// `LambertW` is only defined for `z >= -1/e`; below that libxc calls
/// `exit(1)`, so the sweep stops at the branch point.
#[test]
fn lambert_w_is_bit_identical_to_libxc() {
    let mut rng = Rng(0x13579BDF2468ACE0);
    let inv_e = 1.0 / std::f64::consts::E;
    let mut vals = vec![
        -inv_e, -0.35, -0.3140862435046707, -0.1, -1e-6, 0.0, 1e-6, 0.5, 1.0,
        1.149876485041417, 2.0, 10.0, 100.0, 1e5,
    ];
    for _ in 0..200_000 {
        vals.push(rng.uniform(-inv_e, 100.0));
    }
    for _ in 0..50_000 {
        vals.push(rng.logmag(1e-12, 1e10, false));
    }
    // The branch point itself is excluded against a contracted oracle, and
    // only there. `w = sqrt(2*M_E*z + 2) - 1` is exactly `-1` when the
    // expression is evaluated as written, which converges immediately;
    // contracted, `fma(2*M_E, z, 2.0)` is 3.88e-17 rather than 0, so libxc
    // starts from -0.99999999377 instead, fails to converge in its fifteen
    // Halley steps, and returns its "should never happen" 0.0. That is a
    // branch flip, not a tolerance, so no bound covers it -- with
    // LIBXC_RS_FP_CONTRACT=off this point agrees bitwise like every other.
    let branch = -1.0 / std::f64::consts::E;
    let skip_branch = !contraction_off();
    let d = sweep(&vals, libxc_rkernel_math::lambert_w::lambert_w, LambertW, |x| {
        skip_branch && (x - branch).abs() < 1e-16
    });
    check("LambertW", &d, 1e-14, "Halley's own convergence criterion");
}
