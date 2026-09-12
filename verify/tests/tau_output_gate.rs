//! Every MGGA without `XC_FLAGS_NEEDS_TAU`, against libxc, bit for bit.
//!
//! libxc's maple2c code writes each output whose name contains `tau` only
//! under `p->info->flags & XC_FLAGS_NEEDS_TAU` (and each `lapl` one only under
//! `XC_FLAGS_NEEDS_LAPLACIAN`): 101,769 guarded writes across the meta-GGA
//! maple2c files, without exception. `tools/translate_rayon/from_maple.py`
//! drops output guards, so `gen_eval.py` re-applies the tau one in the
//! dispatch (`ZERO_TAU`, see `_tau_output_gate`). One functional needs it
//! today: `mgga_x_2d_prhg07_prp10` has no `NEEDS_TAU` but its body reads
//! `tau`, and before 2026-09-11 it returned nonzero `v2rhotau`, `v3rho2tau`,
//! `v3rhotau2`, ... where libxc returns 0. `kernel_oracle.rs` skips it as a 2D
//! functional, so nothing saw it.
//!
//! This compares every output field of every MGGA without `NEEDS_TAU`, at
//! every order it claims, in both spins. The deorbitalized SCAN-L family is
//! left to `deorbital_oracle.rs`, which knows which six of libxc's own lxc
//! outputs are uninitialised memory for it.

use libxc_rs::Dimensions;
use libxc_rs::eval::workspace::EvaluationWorkspace;
use libxc_rs::functional::Functional;
use libxc_rs::input::MggaInput;
use libxc_rs::model::{DerivativeOrder, Family, FunctionalFlags, Spin};
use libxc_rs::output::MggaOutput;
use libxc_rs::registry::all_functional_ids;
use libxc_sys::{XC_POLARIZED, XC_UNPOLARIZED, xc_func_end, xc_func_init, xc_func_type, xc_mgga};

/// Covered by `deorbital_oracle.rs` instead: libxc leaves six of their lxc
/// outputs as uninitialised memory, which a bit-for-bit comparison cannot use.
const DEORBITALIZED: [u16; 7] = [700, 701, 702, 703, 704, 718, 719];

/// libxc's MGGA output fields, in `xc_mgga`'s argument order.
const NAMES: [&str; 70] = [
    "zk",
    "vrho",
    "vsigma",
    "vlapl",
    "vtau",
    "v2rho2",
    "v2rhosigma",
    "v2rholapl",
    "v2rhotau",
    "v2sigma2",
    "v2sigmalapl",
    "v2sigmatau",
    "v2lapl2",
    "v2lapltau",
    "v2tau2",
    "v3rho3",
    "v3rho2sigma",
    "v3rho2lapl",
    "v3rho2tau",
    "v3rhosigma2",
    "v3rhosigmalapl",
    "v3rhosigmatau",
    "v3rholapl2",
    "v3rholapltau",
    "v3rhotau2",
    "v3sigma3",
    "v3sigma2lapl",
    "v3sigma2tau",
    "v3sigmalapl2",
    "v3sigmalapltau",
    "v3sigmatau2",
    "v3lapl3",
    "v3lapl2tau",
    "v3lapltau2",
    "v3tau3",
    "v4rho4",
    "v4rho3sigma",
    "v4rho3lapl",
    "v4rho3tau",
    "v4rho2sigma2",
    "v4rho2sigmalapl",
    "v4rho2sigmatau",
    "v4rho2lapl2",
    "v4rho2lapltau",
    "v4rho2tau2",
    "v4rhosigma3",
    "v4rhosigma2lapl",
    "v4rhosigma2tau",
    "v4rhosigmalapl2",
    "v4rhosigmalapltau",
    "v4rhosigmatau2",
    "v4rholapl3",
    "v4rholapl2tau",
    "v4rholapltau2",
    "v4rhotau3",
    "v4sigma4",
    "v4sigma3lapl",
    "v4sigma3tau",
    "v4sigma2lapl2",
    "v4sigma2lapltau",
    "v4sigma2tau2",
    "v4sigmalapl3",
    "v4sigmalapl2tau",
    "v4sigmalapltau2",
    "v4sigmatau3",
    "v4lapl4",
    "v4lapl3tau",
    "v4lapl2tau2",
    "v4lapltau3",
    "v4tau4",
];

/// The derivative order field `k` of [`NAMES`] belongs to.
fn order_of(k: usize) -> u8 {
    match k {
        0 => 0,
        1..=4 => 1,
        5..=14 => 2,
        15..=34 => 3,
        _ => 4,
    }
}

fn widths(d: &Dimensions) -> [usize; 70] {
    macro_rules! w { ($($f:ident),+) => { [$(d.$f as usize),+] } }
    w!(
        zk,
        vrho,
        vsigma,
        vlapl,
        vtau,
        v2rho2,
        v2rhosigma,
        v2rholapl,
        v2rhotau,
        v2sigma2,
        v2sigmalapl,
        v2sigmatau,
        v2lapl2,
        v2lapltau,
        v2tau2,
        v3rho3,
        v3rho2sigma,
        v3rho2lapl,
        v3rho2tau,
        v3rhosigma2,
        v3rhosigmalapl,
        v3rhosigmatau,
        v3rholapl2,
        v3rholapltau,
        v3rhotau2,
        v3sigma3,
        v3sigma2lapl,
        v3sigma2tau,
        v3sigmalapl2,
        v3sigmalapltau,
        v3sigmatau2,
        v3lapl3,
        v3lapl2tau,
        v3lapltau2,
        v3tau3,
        v4rho4,
        v4rho3sigma,
        v4rho3lapl,
        v4rho3tau,
        v4rho2sigma2,
        v4rho2sigmalapl,
        v4rho2sigmatau,
        v4rho2lapl2,
        v4rho2lapltau,
        v4rho2tau2,
        v4rhosigma3,
        v4rhosigma2lapl,
        v4rhosigma2tau,
        v4rhosigmalapl2,
        v4rhosigmalapltau,
        v4rhosigmatau2,
        v4rholapl3,
        v4rholapl2tau,
        v4rholapltau2,
        v4rhotau3,
        v4sigma4,
        v4sigma3lapl,
        v4sigma3tau,
        v4sigma2lapl2,
        v4sigma2lapltau,
        v4sigma2tau2,
        v4sigmalapl3,
        v4sigmalapl2tau,
        v4sigmalapltau2,
        v4sigmatau3,
        v4lapl4,
        v4lapl3tau,
        v4lapl2tau2,
        v4lapltau3,
        v4tau4
    )
}

/// Zeroed buffers for every field of an order this functional CLAIMS, up to
/// `order`; empty (a null pointer to libxc) for every other field.
///
/// "Up to `order`" alone is not enough: a potential-only functional claims
/// vxc upward but not exc, and handing libxc a non-null `zk` for it is fatal
/// -- `xc_mgga` prints "Functional '...' does not provide an implementation of
/// Exc" and exits the process, taking the test binary with it.
fn buffers(
    d: &Dimensions,
    np: usize,
    order: DerivativeOrder,
    claimed: &[DerivativeOrder],
) -> Vec<Vec<f64>> {
    widths(d)
        .iter()
        .enumerate()
        .map(|(k, w)| {
            let o = order_of(k);
            if o <= order as u8 && claimed.iter().any(|c| *c as u8 == o) {
                vec![0.0; w * np]
            } else {
                Vec::new()
            }
        })
        .collect()
}

fn output(b: &mut [Vec<f64>]) -> MggaOutput<'_> {
    let mut it = b.iter_mut().map(|v| {
        if v.is_empty() {
            None
        } else {
            Some(v.as_mut_slice())
        }
    });
    let mut nx = || it.next().unwrap();
    MggaOutput {
        zk: nx(),
        vrho: nx(),
        vsigma: nx(),
        vlapl: nx(),
        vtau: nx(),
        v2rho2: nx(),
        v2rhosigma: nx(),
        v2rholapl: nx(),
        v2rhotau: nx(),
        v2sigma2: nx(),
        v2sigmalapl: nx(),
        v2sigmatau: nx(),
        v2lapl2: nx(),
        v2lapltau: nx(),
        v2tau2: nx(),
        v3rho3: nx(),
        v3rho2sigma: nx(),
        v3rho2lapl: nx(),
        v3rho2tau: nx(),
        v3rhosigma2: nx(),
        v3rhosigmalapl: nx(),
        v3rhosigmatau: nx(),
        v3rholapl2: nx(),
        v3rholapltau: nx(),
        v3rhotau2: nx(),
        v3sigma3: nx(),
        v3sigma2lapl: nx(),
        v3sigma2tau: nx(),
        v3sigmalapl2: nx(),
        v3sigmalapltau: nx(),
        v3sigmatau2: nx(),
        v3lapl3: nx(),
        v3lapl2tau: nx(),
        v3lapltau2: nx(),
        v3tau3: nx(),
        v4rho4: nx(),
        v4rho3sigma: nx(),
        v4rho3lapl: nx(),
        v4rho3tau: nx(),
        v4rho2sigma2: nx(),
        v4rho2sigmalapl: nx(),
        v4rho2sigmatau: nx(),
        v4rho2lapl2: nx(),
        v4rho2lapltau: nx(),
        v4rho2tau2: nx(),
        v4rhosigma3: nx(),
        v4rhosigma2lapl: nx(),
        v4rhosigma2tau: nx(),
        v4rhosigmalapl2: nx(),
        v4rhosigmalapltau: nx(),
        v4rhosigmatau2: nx(),
        v4rholapl3: nx(),
        v4rholapl2tau: nx(),
        v4rholapltau2: nx(),
        v4rhotau3: nx(),
        v4sigma4: nx(),
        v4sigma3lapl: nx(),
        v4sigma3tau: nx(),
        v4sigma2lapl2: nx(),
        v4sigma2lapltau: nx(),
        v4sigma2tau2: nx(),
        v4sigmalapl3: nx(),
        v4sigmalapl2tau: nx(),
        v4sigmalapltau2: nx(),
        v4sigmatau3: nx(),
        v4lapl4: nx(),
        v4lapl3tau: nx(),
        v4lapl2tau2: nx(),
        v4lapltau3: nx(),
        v4tau4: nx(),
    }
}

struct CFunc(xc_func_type);
impl Drop for CFunc {
    fn drop(&mut self) {
        unsafe { xc_func_end(&mut self.0) };
    }
}

fn c_eval(cf: &CFunc, np: usize, g: &(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), b: &mut [Vec<f64>]) {
    let p: Vec<*mut f64> = b
        .iter_mut()
        .map(|v| {
            if v.is_empty() {
                std::ptr::null_mut()
            } else {
                v.as_mut_ptr()
            }
        })
        .collect();
    macro_rules! call {
        ($($i:literal),+) => {
            unsafe { xc_mgga(&cf.0, np, g.0.as_ptr(), g.1.as_ptr(), g.2.as_ptr(), g.3.as_ptr(), $(p[$i]),+) }
        };
    }
    call!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69
    );
}

/// A physical grid: densities over the chemically active band, reduced
/// gradients of order one, a Laplacian of either sign, and a caller `tau`
/// above the von Weizsaecker bound -- nonzero, so a kernel that should be
/// evaluated at `tau = 0` but is handed the caller's value shows up.
fn grid(np: usize, nspin: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut s = 0x7a0_9a7e_5eed_1234u64;
    let mut next = || {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let (mut rho, mut sigma, mut lapl, mut tau) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for _ in 0..np {
        let mut r = [0.0f64; 2];
        let mut g = [0.0f64; 2];
        for k in 0..nspin {
            r[k] = 10f64.powf(-4.0 + 5.0 * next());
            g[k] = r[k].powf(4.0 / 3.0) * (0.1 + 2.0 * next());
            rho.push(r[k]);
        }
        if nspin == 1 {
            sigma.push(g[0] * g[0]);
        } else {
            sigma.extend([g[0] * g[0], g[0] * g[1] * (2.0 * next() - 1.0), g[1] * g[1]]);
        }
        for k in 0..nspin {
            tau.push(g[k] * g[k] / (8.0 * r[k]) * (1.05 + 5.0 * next()));
            lapl.push(r[k].powf(5.0 / 3.0) * (2.0 * next() - 1.0));
        }
    }
    (rho, sigma, lapl, tau)
}

fn claimed(flags: FunctionalFlags) -> Vec<DerivativeOrder> {
    [
        (DerivativeOrder::Exc, FunctionalFlags::HAVE_EXC),
        (DerivativeOrder::Vxc, FunctionalFlags::HAVE_VXC),
        (DerivativeOrder::Fxc, FunctionalFlags::HAVE_FXC),
        (DerivativeOrder::Kxc, FunctionalFlags::HAVE_KXC),
        (DerivativeOrder::Lxc, FunctionalFlags::HAVE_LXC),
    ]
    .into_iter()
    .filter(|(_, f)| flags.contains(*f))
    .map(|(o, _)| o)
    .collect()
}

#[test]
fn mgga_without_needs_tau_matches_libxc_bit_for_bit() {
    let np = 150usize;
    let mut checked = Vec::new();
    let mut bad = Vec::new();
    let mut compared = 0usize;
    for id in all_functional_ids() {
        let meta = id.meta();
        if !matches!(meta.family, Family::Mgga)
            || meta.flags.contains(FunctionalFlags::NEEDS_TAU)
            || DEORBITALIZED.contains(&id.raw())
        {
            continue;
        }
        checked.push(meta.name);
        for spin in [Spin::Unpolarized, Spin::Polarized] {
            let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
            let g = grid(np, nspin);
            let d = Dimensions::mgga(spin);
            let w = widths(&d);
            let orders = claimed(meta.flags);
            for &order in &orders {
                let f = Functional::new(id, spin).unwrap_or_else(|e| panic!("{}: {e}", meta.name));
                let input = MggaInput::new(&g.0, &g.1, &g.2, &g.3, np, spin).unwrap();
                let mut ours = buffers(&d, np, order, &orders);
                let mut ws = EvaluationWorkspace::new(np, spin);
                f.evaluate_mgga(&input, order, &mut output(&mut ours), &mut ws)
                    .unwrap_or_else(|e| panic!("{} {spin:?} {order:?}: {e}", meta.name));

                let mut t: xc_func_type = unsafe { std::mem::zeroed() };
                let n = if nspin == 1 {
                    XC_UNPOLARIZED
                } else {
                    XC_POLARIZED
                } as i32;
                assert_eq!(
                    unsafe { xc_func_init(&mut t, id.raw() as i32, n) },
                    0,
                    "{}",
                    meta.name
                );
                let cf = CFunc(t);
                let mut theirs = buffers(&d, np, order, &orders);
                c_eval(&cf, np, &g, &mut theirs);

                for k in 0..NAMES.len() {
                    if order_of(k) != order as u8 || w[k] == 0 || ours[k].is_empty() {
                        continue;
                    }
                    compared += ours[k].len();
                    let diff = ours[k]
                        .iter()
                        .zip(&theirs[k])
                        .filter(|(x, y)| x.to_bits() != y.to_bits())
                        .count();
                    if diff > 0 {
                        bad.push(format!(
                            "{} {spin:?} {order:?} {}: {diff} of {} values differ",
                            meta.name,
                            NAMES[k],
                            ours[k].len()
                        ));
                    }
                }
            }
        }
    }
    println!(
        "{} MGGA functionals without NEEDS_TAU: {checked:?}",
        checked.len()
    );
    println!("{compared} values compared bit for bit");
    assert!(
        checked
            .iter()
            .any(|n| n.eq_ignore_ascii_case("XC_MGGA_X_2D_PRHG07_PRP10")),
        "the functional this gate exists for is not in the set: {checked:?}"
    );
    assert!(
        bad.is_empty(),
        "{} field(s) differ from libxc:\n  {}",
        bad.len(),
        bad.join("\n  ")
    );
}
