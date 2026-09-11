//! The deorbitalized SCAN-L family against libxc, every order, both spins.
//!
//! libxc builds `mgga_x_scanl`, `mgga_x_revscanl`, `mgga_c_scanl`,
//! `mgga_x_r2scanl` and `mgga_c_r2scanl` with `xc_deorbitalize_init` and
//! evaluates them with `xc_deorbitalize_func`: a kinetic-energy functional
//! stands in for `tau`, and every derivative is chain-ruled through it
//! (`maple2c/deorbitalize_N.c`). `mgga_c_scanl_rvv10` / `_vv10` are mixes over
//! `mgga_c_scanl`. Until 2026-09-11 all seven refused every order.
//!
//! This compares every output field of every order each claims against
//! libxc's own `xc_mgga`, and once more at non-default ext_params, which is
//! what shows the parent's parameters reach the right auxiliary.

use libxc_rs::eval::workspace::EvaluationWorkspace;
use libxc_rs::functional::Functional;
use libxc_rs::input::MggaInput;
use libxc_rs::model::{DerivativeOrder, FunctionalFlags, FunctionalId, Spin};
use libxc_rs::output::MggaOutput;
use libxc_rs::registry::lookup_by_id;
use libxc_rs::Dimensions;
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_set_ext_params, xc_func_type, xc_mgga, XC_POLARIZED,
    XC_UNPOLARIZED,
};

/// The five deorbitalized functionals, and the two mixes over one of them.
const IDS: [u16; 7] = [700, 701, 702, 703, 704, 718, 719];

/// Fourth-derivative fields libxc's own deorbitalization computes from
/// uninitialized memory, so no comparison against it means anything.
///
/// `xc_mgga_vars_allocate_all` (`deorbitalize_func.c:188-189`) mallocs the base
/// meta-GGA's `v4sigmalapltau2` buffer and then memsets `v4sigmalapl2tau` a
/// second time instead of it. `xc_mgga` does not repair that: it zeroes a
/// lapl-tau cross field only for a functional with both `NEEDS_LAPLACIAN` and
/// `NEEDS_TAU` (`mgga.c:262`), and none of the SCAN-L bases needs the
/// Laplacian -- so the kernel, which accumulates with `+=`, adds nothing to
/// whatever the heap held, and `maple2c/deorbitalize_4.c` reads
/// `mgga_v4sigmalapltau2` into exactly these six outputs. On a fresh heap it
/// happens to be zero and the two libraries agree bit for bit; after a sweep
/// has churned the allocator, libxc returns noise (`v4sigma3lapl` 1.04
/// relative, `mgga_c_scanl_vv10`, polarized, 2026-09-11). This tree zeroes the
/// buffer, which is what libxc's allocator plainly means to do.
const LIBXC_UNZEROED: [&str; 6] = [
    "v4rho2sigmalapl", "v4rhosigma2lapl", "v4rhosigmalapl2",
    "v4sigma3lapl", "v4sigma2lapl2", "v4sigmalapl3",
];

/// Energy density: the project's contract.
const TOL_ZK: f64 = 1e-12;
/// Derivatives, pointwise-scaled (see [`worst_pointwise`]).
const TOL_V: f64 = 1e-9;

const FIELDS: [(&str, u8); 70] = [
    ("zk", 0), ("vrho", 1), ("vsigma", 1), ("vlapl", 1), ("vtau", 1),
    ("v2rho2", 2), ("v2rhosigma", 2), ("v2rholapl", 2), ("v2rhotau", 2), ("v2sigma2", 2),
    ("v2sigmalapl", 2), ("v2sigmatau", 2), ("v2lapl2", 2), ("v2lapltau", 2), ("v2tau2", 2),
    ("v3rho3", 3), ("v3rho2sigma", 3), ("v3rho2lapl", 3), ("v3rho2tau", 3), ("v3rhosigma2", 3),
    ("v3rhosigmalapl", 3), ("v3rhosigmatau", 3), ("v3rholapl2", 3), ("v3rholapltau", 3),
    ("v3rhotau2", 3), ("v3sigma3", 3), ("v3sigma2lapl", 3), ("v3sigma2tau", 3),
    ("v3sigmalapl2", 3), ("v3sigmalapltau", 3), ("v3sigmatau2", 3), ("v3lapl3", 3),
    ("v3lapl2tau", 3), ("v3lapltau2", 3), ("v3tau3", 3),
    ("v4rho4", 4), ("v4rho3sigma", 4), ("v4rho3lapl", 4), ("v4rho3tau", 4), ("v4rho2sigma2", 4),
    ("v4rho2sigmalapl", 4), ("v4rho2sigmatau", 4), ("v4rho2lapl2", 4), ("v4rho2lapltau", 4),
    ("v4rho2tau2", 4), ("v4rhosigma3", 4), ("v4rhosigma2lapl", 4), ("v4rhosigma2tau", 4),
    ("v4rhosigmalapl2", 4), ("v4rhosigmalapltau", 4), ("v4rhosigmatau2", 4), ("v4rholapl3", 4),
    ("v4rholapl2tau", 4), ("v4rholapltau2", 4), ("v4rhotau3", 4), ("v4sigma4", 4),
    ("v4sigma3lapl", 4), ("v4sigma3tau", 4), ("v4sigma2lapl2", 4), ("v4sigma2lapltau", 4),
    ("v4sigma2tau2", 4), ("v4sigmalapl3", 4), ("v4sigmalapl2tau", 4), ("v4sigmalapltau2", 4),
    ("v4sigmatau3", 4), ("v4lapl4", 4), ("v4lapl3tau", 4), ("v4lapl2tau2", 4),
    ("v4lapltau3", 4), ("v4tau4", 4),
];

fn widths(d: &Dimensions) -> [usize; 70] {
    macro_rules! w { ($($f:ident),+) => { [$(d.$f as usize),+] } }
    w!(zk, vrho, vsigma, vlapl, vtau,
       v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2,
       v2lapltau, v2tau2,
       v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau,
       v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2,
       v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3,
       v4rho4, v4rho3sigma, v4rho3lapl, v4rho3tau, v4rho2sigma2, v4rho2sigmalapl, v4rho2sigmatau,
       v4rho2lapl2, v4rho2lapltau, v4rho2tau2, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau,
       v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau,
       v4rholapltau2, v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2,
       v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2,
       v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4)
}

fn buffers(d: &Dimensions, np: usize, order: DerivativeOrder) -> Vec<Vec<f64>> {
    widths(d)
        .iter()
        .zip(FIELDS.iter())
        .map(|(w, (_, o))| if *o <= order as u8 { vec![0.0; w * np] } else { Vec::new() })
        .collect()
}

fn output(b: &mut [Vec<f64>]) -> MggaOutput<'_> {
    let mut it = b.iter_mut().map(|v| if v.is_empty() { None } else { Some(v.as_mut_slice()) });
    let mut nx = || it.next().unwrap();
    MggaOutput {
        zk: nx(), vrho: nx(), vsigma: nx(), vlapl: nx(), vtau: nx(),
        v2rho2: nx(), v2rhosigma: nx(), v2rholapl: nx(), v2rhotau: nx(), v2sigma2: nx(),
        v2sigmalapl: nx(), v2sigmatau: nx(), v2lapl2: nx(), v2lapltau: nx(), v2tau2: nx(),
        v3rho3: nx(), v3rho2sigma: nx(), v3rho2lapl: nx(), v3rho2tau: nx(), v3rhosigma2: nx(),
        v3rhosigmalapl: nx(), v3rhosigmatau: nx(), v3rholapl2: nx(), v3rholapltau: nx(),
        v3rhotau2: nx(), v3sigma3: nx(), v3sigma2lapl: nx(), v3sigma2tau: nx(),
        v3sigmalapl2: nx(), v3sigmalapltau: nx(), v3sigmatau2: nx(), v3lapl3: nx(),
        v3lapl2tau: nx(), v3lapltau2: nx(), v3tau3: nx(),
        v4rho4: nx(), v4rho3sigma: nx(), v4rho3lapl: nx(), v4rho3tau: nx(), v4rho2sigma2: nx(),
        v4rho2sigmalapl: nx(), v4rho2sigmatau: nx(), v4rho2lapl2: nx(), v4rho2lapltau: nx(),
        v4rho2tau2: nx(), v4rhosigma3: nx(), v4rhosigma2lapl: nx(), v4rhosigma2tau: nx(),
        v4rhosigmalapl2: nx(), v4rhosigmalapltau: nx(), v4rhosigmatau2: nx(), v4rholapl3: nx(),
        v4rholapl2tau: nx(), v4rholapltau2: nx(), v4rhotau3: nx(), v4sigma4: nx(),
        v4sigma3lapl: nx(), v4sigma3tau: nx(), v4sigma2lapl2: nx(), v4sigma2lapltau: nx(),
        v4sigma2tau2: nx(), v4sigmalapl3: nx(), v4sigmalapl2tau: nx(), v4sigmalapltau2: nx(),
        v4sigmatau3: nx(), v4lapl4: nx(), v4lapl3tau: nx(), v4lapl2tau2: nx(),
        v4lapltau3: nx(), v4tau4: nx(),
    }
}

struct CFunc(xc_func_type);
impl Drop for CFunc {
    fn drop(&mut self) {
        unsafe { xc_func_end(&mut self.0) };
    }
}

fn c_eval(
    cf: &CFunc,
    np: usize,
    g: &(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    b: &mut [Vec<f64>],
) {
    let p: Vec<*mut f64> = b
        .iter_mut()
        .map(|v| if v.is_empty() { std::ptr::null_mut() } else { v.as_mut_ptr() })
        .collect();
    macro_rules! call {
        ($($i:literal),+) => {
            unsafe { xc_mgga(&cf.0, np, g.0.as_ptr(), g.1.as_ptr(), g.2.as_ptr(), g.3.as_ptr(), $(p[$i]),+) }
        };
    }
    call!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
          24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
          46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
          68, 69);
}

/// A physical grid: densities over the chemically active band, reduced
/// gradients of order one, a Laplacian of either sign and `tau` above the von
/// Weizsaecker bound (the base meta-GGA sees the deorbitalized `tau`, not
/// this one, but the kinetic-energy functional and the grid should be sane).
fn grid(np: usize, nspin: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut s = 0x0dd0_7ab1_e5ca_1e11u64;
    let mut next = || {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
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

/// Worst difference over one field, each point measured against the largest
/// libxc component at that point.
fn worst_pointwise(ours: &[f64], theirs: &[f64], width: usize) -> f64 {
    let mut w = 0.0f64;
    for (a, b) in ours.chunks(width.max(1)).zip(theirs.chunks(width.max(1))) {
        let ps = b.iter().fold(0.0f64, |m, v| m.max(v.abs())).max(f64::MIN_POSITIVE);
        for (x, y) in a.iter().zip(b.iter()) {
            if x == y || !x.is_finite() || !y.is_finite() {
                continue;
            }
            w = w.max((x - y).abs() / ps);
        }
    }
    w
}

/// One (functional, spin, order) comparison: `(worst field, worst, nonzero
/// libxc values compared, of which bit-identical)`.
fn compare(
    id: u16,
    spin: Spin,
    order: DerivativeOrder,
    ext: Option<&[f64]>,
) -> (&'static str, f64, usize, usize) {
    let np = 150usize;
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let g = grid(np, nspin);
    let d = Dimensions::mgga(spin);
    let w = widths(&d);

    let mut f = Functional::new(FunctionalId::from_raw(id).unwrap(), spin).unwrap();
    if let Some(e) = ext {
        f.set_ext_params(e).unwrap();
    }
    let input = MggaInput::new(&g.0, &g.1, &g.2, &g.3, np, spin).unwrap();
    let mut ours = buffers(&d, np, order);
    let mut ws = EvaluationWorkspace::new(np, spin);
    f.evaluate_mgga(&input, order, &mut output(&mut ours), &mut ws)
        .unwrap_or_else(|e| panic!("{id} {spin:?} {order:?}: {e}"));

    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    let n = if nspin == 1 { XC_UNPOLARIZED } else { XC_POLARIZED } as i32;
    assert_eq!(unsafe { xc_func_init(&mut t, id as i32, n) }, 0);
    let cf = CFunc(t);
    if let Some(e) = ext {
        unsafe { xc_func_set_ext_params(&cf.0 as *const _ as *mut _, e.as_ptr()) };
    }
    let mut theirs = buffers(&d, np, order);
    c_eval(&cf, np, &g, &mut theirs);

    let (mut worst, mut field, mut nonzero, mut same) = (0.0f64, "-", 0usize, 0usize);
    for (k, (name, o)) in FIELDS.iter().enumerate() {
        if *o != order as u8 || w[k] == 0 {
            continue;
        }
        if order == DerivativeOrder::Lxc && LIBXC_UNZEROED.contains(name) {
            assert!(ours[k].iter().all(|v| v.is_finite()), "{id} {name}: not finite");
            continue;
        }
        for (x, y) in ours[k].iter().zip(theirs[k].iter()) {
            if *y != 0.0 {
                nonzero += 1;
                same += usize::from(x.to_bits() == y.to_bits());
            }
        }
        let e = worst_pointwise(&ours[k], &theirs[k], w[k]);
        if e > worst {
            worst = e;
            field = name;
        }
    }
    (field, worst, nonzero, same)
}

fn claimed(id: u16) -> Vec<DerivativeOrder> {
    let m = lookup_by_id(id).unwrap();
    [
        (DerivativeOrder::Exc, FunctionalFlags::HAVE_EXC),
        (DerivativeOrder::Vxc, FunctionalFlags::HAVE_VXC),
        (DerivativeOrder::Fxc, FunctionalFlags::HAVE_FXC),
        (DerivativeOrder::Kxc, FunctionalFlags::HAVE_KXC),
        (DerivativeOrder::Lxc, FunctionalFlags::HAVE_LXC),
    ]
    .into_iter()
    .filter(|(_, f)| m.flags.contains(*f))
    .map(|(o, _)| o)
    .collect()
}

fn gate(rows: &[(String, &'static str, f64, usize, usize, DerivativeOrder)]) -> Vec<String> {
    let mut bad = Vec::new();
    for (what, field, worst, nonzero, _, order) in rows {
        let tol = if *order == DerivativeOrder::Exc { TOL_ZK } else { TOL_V };
        if *worst > tol {
            bad.push(format!("{what}: {field} {worst:.3e}"));
        }
        if *nonzero == 0 {
            bad.push(format!("{what}: compared no nonzero values"));
        }
    }
    bad
}

#[test]
fn deorbitalized_family_matches_libxc() {
    let mut rows = Vec::new();
    for id in IDS {
        let name = lookup_by_id(id).unwrap().name.to_lowercase();
        for spin in [Spin::Unpolarized, Spin::Polarized] {
            for order in claimed(id) {
                let (field, worst, nz, eq) = compare(id, spin, order, None);
                println!(
                    "{name:<26} {spin:<11?} {order:?}  {nz:>7} nonzero, {eq:>7} bit-identical, \
                     worst {field} {worst:.3e}"
                );
                rows.push((format!("{name} {spin:?} {order:?}"), field, worst, nz, eq, order));
            }
        }
    }
    let bad = gate(&rows);
    assert!(bad.is_empty(), "{} over the gate:\n{}", bad.len(), bad.join("\n"));
}

/// The parent's ext_params land on the right auxiliary: SCAN-L's first four
/// are SCAN's, its last two PC07's; `mgga_c_scanl`'s two are PC07's. Moved
/// off their defaults, both libraries must still agree.
#[test]
fn deorbitalized_ext_params_reach_their_auxiliaries() {
    let cases: [(u16, &[f64]); 4] = [
        (700, &[0.62, 0.75, 1.30, 0.070, 1.70, 0.27]),
        (702, &[1.70, 0.27]),
        (718, &[0.62, 0.75, 1.30, 0.070, 0.002, 0.35, 1.70, 0.27]),
        (719, &[1.70, 0.27]),
    ];
    let mut rows = Vec::new();
    for (id, ext) in cases {
        let name = lookup_by_id(id).unwrap().name.to_lowercase();
        for spin in [Spin::Unpolarized, Spin::Polarized] {
            for order in [DerivativeOrder::Exc, DerivativeOrder::Vxc] {
                let (field, worst, nz, eq) = compare(id, spin, order, Some(ext));
                let (_, dflt, _, _) = compare(id, spin, order, None);
                println!("{name:<22} {spin:<11?} {order:?} ext  {nz} nonzero, {eq} bit-identical, worst {worst:.3e}");
                rows.push((format!("{name} {spin:?} {order:?} ext"), field, worst, nz, eq, order));
                let _ = dflt;
            }
        }
    }
    let bad = gate(&rows);
    assert!(bad.is_empty(), "{} over the gate:\n{}", bad.len(), bad.join("\n"));
}
