//! Every public functional, both spins, every derivative order its own flags
//! claim: does the public Rust API evaluate it, or refuse?
//!
//! This is the gate for `docs/PLAN-defect-remediation-v6.md`. It checks
//! return codes, not values -- every item that removes a refusal carries its
//! own oracle test for the numbers it unlocks. What this test adds is that a
//! refusal cannot appear, or disappear, without someone deciding so:
//!
//! - a refusal that is not in [`ALLOWED`] fails the test, and
//! - an [`ALLOWED`] row that now evaluates fails the test too, so the list can
//!   only shrink, and must be shrunk in the commit that fixes it.
//!
//! It goes through [`BatchEvaluator`], the entry point `pyscf_rs` calls. The
//! C-ABI sweep in the plan's section 3 is the reproduction; the two must
//! agree.
//!
//! `lda_k_gds08_worker` (libxc number 100001) is not in the public registry,
//! so it never enters the sweep: a functional without a public id is
//! unsupported by design.
//!
//! `REFUSAL_SWEEP_PRINT=1` prints every refusal as an allowlist row.

use std::collections::BTreeSet;

use libxc_rs::model::{DerivativeOrder, Family, FunctionalFlags, Spin};
use libxc_rs::registry::all_functional_ids;
use libxc_rs::{
    BatchEvaluator, Dimensions, Functional, GgaInput, GgaOutput, LdaInput, LdaOutput, MggaInput,
    MggaOutput,
};

/// Refusals that are known, each tagged with the plan item that removes it.
///
/// `"init"` is a refusal at construction; the others are derivative orders.
/// A row covers both spins: every refusal so far has been spin-independent,
/// and the test fails if one ever is not.
const ALLOWED: &[(&str, &str, &str)] = &[
];

const ORDERS: [(DerivativeOrder, &str, FunctionalFlags); 5] = [
    (DerivativeOrder::Exc, "exc", FunctionalFlags::HAVE_EXC),
    (DerivativeOrder::Vxc, "vxc", FunctionalFlags::HAVE_VXC),
    (DerivativeOrder::Fxc, "fxc", FunctionalFlags::HAVE_FXC),
    (DerivativeOrder::Kxc, "kxc", FunctionalFlags::HAVE_KXC),
    (DerivativeOrder::Lxc, "lxc", FunctionalFlags::HAVE_LXC),
];

const NP: usize = 4;

/// Derivative order an output field belongs to, from its name.
fn rank(field: &str) -> u8 {
    match field.as_bytes() {
        b"zk" => 0,
        [b'v', d, ..] if d.is_ascii_digit() => d - b'0',
        _ => 1,
    }
}

/// Allocate every output field of `order` and below that the family has,
/// build the family's output struct over them, and run `$body` with it bound
/// to `$out`. `zk` is left out when the functional does not claim an energy
/// (the potential-only functionals), which is how libxc's own `xc_*_vxc`
/// entry points call them.
macro_rules! with_output {
    ($Out:ident, $d:expr, $order:expr, $have_exc:expr, |$out:ident| $body:expr; $($f:ident),* $(,)?) => {{
        $(
            let wanted = rank(stringify!($f)) <= $order as u8
                && (stringify!($f) != "zk" || $have_exc);
            let mut $f: Vec<f64> = if wanted { vec![0.0; $d.$f as usize * NP] } else { Vec::new() };
        )*
        let mut $out = $Out {
            $( $f: if $f.is_empty() { None } else { Some(&mut $f[..]) }, )*
        };
        $body
    }};
}

/// A small physical grid: positive densities, `sigma_ab` inside
/// `+-(sigma_aa + sigma_bb)/2`, `tau` above the von Weizsaecker bound.
fn inputs(spin: Spin) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let ns = if spin == Spin::Polarized { 2 } else { 1 };
    let nsig = if ns == 2 { 3 } else { 1 };
    let mut rho = Vec::new();
    let mut sigma = Vec::new();
    let mut lapl = Vec::new();
    let mut tau = Vec::new();
    for ip in 0..NP {
        let r = 0.05 + 0.1 * ip as f64;
        for s in 0..ns {
            rho.push(r * (1.0 + 0.2 * s as f64));
            lapl.push(0.03 * r);
        }
        let g = 0.4 * r.powf(4.0 / 3.0);
        for k in 0..nsig {
            sigma.push(g * g * if k == 1 { 0.3 } else { 1.0 });
        }
        for s in 0..ns {
            let rs = r * (1.0 + 0.2 * s as f64);
            tau.push(g * g / (8.0 * rs) * 1.5 + 0.02);
        }
    }
    (rho, sigma, lapl, tau)
}

fn evaluate(f: &Functional, spin: Spin, order: DerivativeOrder) -> Result<(), String> {
    let meta = f.meta();
    let have_exc = meta.flags.contains(FunctionalFlags::HAVE_EXC);
    let (rho, sigma, lapl, tau) = inputs(spin);
    let mut be = BatchEvaluator::new(spin, NP);
    let r = match meta.family {
        Family::Lda => {
            let input = LdaInput::new(&rho, NP, spin).map_err(|e| e.to_string())?;
            let d = Dimensions::lda(spin);
            with_output!(LdaOutput, d, order, have_exc, |out| be.evaluate(f, &input, order, &mut out);
                zk, vrho, v2rho2, v3rho3, v4rho4)
        }
        Family::Gga => {
            let input = GgaInput::new(&rho, &sigma, NP, spin).map_err(|e| e.to_string())?;
            let d = Dimensions::gga(spin);
            with_output!(GgaOutput, d, order, have_exc, |out| be.evaluate(f, &input, order, &mut out);
                zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
                v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3,
                v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4)
        }
        Family::Mgga => {
            let input =
                MggaInput::new(&rho, &sigma, &lapl, &tau, NP, spin).map_err(|e| e.to_string())?;
            let d = Dimensions::mgga(spin);
            with_output!(MggaOutput, d, order, have_exc, |out| be.evaluate(f, &input, order, &mut out);
                zk, vrho, vsigma, vlapl, vtau,
                v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2,
                v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2,
                v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2,
                v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2,
                v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau,
                v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3,
                v4rho4, v4rho3sigma, v4rho3lapl, v4rho3tau, v4rho2sigma2,
                v4rho2sigmalapl, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2,
                v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau,
                v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3,
                v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau,
                v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3,
                v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4)
        }
    };
    r.map_err(|e| e.to_string())
}

/// `(name, order) -> first error seen`, per spin.
fn sweep(spin: Spin) -> Vec<(String, &'static str, String)> {
    let mut refused = Vec::new();
    for id in all_functional_ids() {
        let meta = id.meta();
        let name = meta.name.trim_start_matches("XC_").to_lowercase();
        let f = match Functional::new(id, spin) {
            Ok(f) => f,
            Err(e) => {
                refused.push((name, "init", e.to_string()));
                continue;
            }
        };
        for (order, label, flag) in ORDERS {
            if !meta.flags.contains(flag) {
                continue;
            }
            // A kernel that panics is a refusal too, and must not take the
            // rest of the sweep with it.
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                evaluate(&f, spin, order)
            }))
            .unwrap_or_else(|_| Err("panicked".to_string()));
            if let Err(e) = r {
                refused.push((name.clone(), label, e));
            }
        }
    }
    refused
}

#[test]
fn public_api_refuses_only_what_is_allowlisted() {
    let unpol = sweep(Spin::Unpolarized);
    let pol = sweep(Spin::Polarized);

    let key = |v: &[(String, &'static str, String)]| -> BTreeSet<(String, &'static str)> {
        v.iter().map(|(n, o, _)| (n.clone(), *o)).collect()
    };
    let (ku, kp) = (key(&unpol), key(&pol));

    if std::env::var_os("REFUSAL_SWEEP_PRINT").is_some() {
        for (n, o, e) in &unpol {
            eprintln!("    ({n:?}, {o:?}, \"?\"), // {e}");
        }
    }
    for label in ["init", "exc", "vxc", "fxc", "kxc", "lxc"] {
        eprintln!(
            "{label}: {} unpolarized, {} polarized",
            ku.iter().filter(|(_, o)| *o == label).count(),
            kp.iter().filter(|(_, o)| *o == label).count(),
        );
    }

    let mut problems = Vec::new();
    for k in ku.symmetric_difference(&kp) {
        problems.push(format!("{} {}: refused in one spin only", k.0, k.1));
    }
    let allowed: BTreeSet<(String, &str)> =
        ALLOWED.iter().map(|(n, o, _)| (n.to_string(), *o)).collect();
    for (n, o, e) in unpol.iter().chain(pol.iter()) {
        if !allowed.contains(&(n.clone(), *o)) {
            problems.push(format!("{n} {o}: refused and not allowlisted: {e}"));
        }
    }
    for (n, o, item) in ALLOWED {
        let k = (n.to_string(), *o);
        if !ku.contains(&k) && !kp.contains(&k) {
            problems.push(format!(
                "{n} {o}: allowlisted for {item} but now evaluates -- remove the row"
            ));
        }
    }
    problems.sort();
    problems.dedup();
    assert!(problems.is_empty(), "{} problem(s):\n{}", problems.len(), problems.join("\n"));
}
