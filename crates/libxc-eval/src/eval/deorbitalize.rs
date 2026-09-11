//! libxc's deorbitalized meta-GGAs: SCAN-L, revSCAN-L, r2SCAN-L and their
//! correlation halves (`xc_deorbitalize_func`, `deorbitalize_func.c`).
//!
//! A deorbitalized functional replaces the orbital-dependent `tau` of its base
//! meta-GGA with a kinetic-energy functional of the density:
//!
//! 1. evaluate the kinetic-energy functional `ked` with no `tau` (libxc passes
//!    `NULL`), once for the whole density when unpolarized, and once per spin
//!    channel -- as a polarized density with the other channel empty -- when
//!    polarized;
//! 2. `tau_s = rho_s * e_ked_s`;
//! 3. evaluate the base meta-GGA at that `tau`;
//! 4. chain-rule every derivative through it: `maple2c/deorbitalize_N.c`,
//!    translated by `tools/translate_rayon/deorbitalize.py` into
//!    [`super::deorbitalize_gen`].
//!
//! The energy is the base's (`*zk = *mgga_zk`), and no `tau` derivative is
//! written: the functional has none. Each auxiliary screens and clamps at its
//! own thresholds, as libxc's `xc_func_init` per auxiliary does, and the parent
//! adds no screen of its own (`deorb_new` has none).

use std::cell::Cell;

use libxc_core::dims::Dimensions;
use libxc_core::error::LibxcRsError;
use libxc_core::input::MggaInput;
use libxc_core::model::{DerivativeOrder, Spin};
use libxc_core::output::MggaOutput;

use crate::eval::deorbitalize_gen as chain;
use crate::eval::dispatch_mgga_by_id;
use crate::eval::mix::mgga_scratch_output;
use crate::eval::workspace::mgga_scratch_view;
use crate::functional::Functional;

thread_local! {
    /// Per-thread scratch for the three evaluations and the derived inputs.
    /// *Taken* out of its cell for the duration of a call rather than
    /// borrowed, so a re-entrant call on the same thread (none today) would
    /// allocate rather than panic; put back afterwards, so steady-state
    /// evaluation allocates nothing.
    static SCRATCH: Cell<Vec<f64>> = const { Cell::new(Vec::new()) };
}

/// `dispatch_mgga_by_id` for a functional that may be deorbitalized. A mix
/// evaluates its meta-GGA auxiliaries through this: a deorbitalized one
/// (`mgga_c_scanl` inside `mgga_c_scanl_vv10`) has no kernel to route to.
pub(crate) fn dispatch_mgga(
    f: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    output: &mut MggaOutput,
) -> Result<(), LibxcRsError> {
    if libxc_core::meta::deorbitalized(f.meta.id).is_some() {
        evaluate(f, input, order, output)
    } else {
        dispatch_mgga_by_id(f.meta.id, input, order, output, f.kernel_ext_params(), &f.thresholds)
    }
}

/// Evaluate a deorbitalized meta-GGA, as `xc_deorbitalize_func_work` does.
///
/// Every output field of `order` and below must be supplied, as `prepare`
/// demands for a kernel; every supplied field is zeroed first, so the ones
/// the chain rule does not write (`vtau` and its relatives) come back zero.
pub(crate) fn evaluate(
    f: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    output: &mut MggaOutput,
) -> Result<(), LibxcRsError> {
    let [base, ked] = f.auxiliaries.as_slice() else {
        return Err(LibxcRsError::UnsupportedFunctional {
            id: f.meta.id,
            reason: "deorbitalized functional built without its base and kinetic-energy auxiliaries",
        });
    };
    let np = input.np();
    let pol = input.spin() == Spin::Polarized;
    let ns = if pol { 2 } else { 1 };
    let d = Dimensions::mgga(input.spin());
    check_and_zero(output, &d, np, order)?;

    let per = d.output_components_through(order) * np;
    let sizes = [
        per,                             // base meta-GGA outputs
        per,                             // ked, spin up (or the whole density)
        if pol { per } else { 0 },       // ked, spin down
        ns * np,                         // the deorbitalized tau
        ns * np,                         // the ked's tau: zero
        if pol { 2 * np } else { 0 },    // one spin channel's rho ...
        if pol { 3 * np } else { 0 },    // ... sigma ...
        if pol { 2 * np } else { 0 },    // ... and lapl, the other channel empty
    ];
    let mut buf = SCRATCH.take();
    buf.clear();
    buf.resize(sizes.iter().sum(), 0.0);
    let r = run(base, ked, input, order, output, &d, &mut buf, &sizes);
    SCRATCH.set(buf);
    r
}

#[allow(clippy::too_many_arguments)]
fn run(
    base: &Functional,
    ked: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    output: &mut MggaOutput,
    d: &Dimensions,
    buf: &mut [f64],
    sizes: &[usize; 8],
) -> Result<(), LibxcRsError> {
    let (np, spin) = (input.np(), input.spin());
    let pol = spin == Spin::Polarized;
    let (rho, sigma, lapl) = (input.rho(), input.sigma(), input.lapl());

    let (bm, rest) = buf.split_at_mut(sizes[0]);
    let (bk1, rest) = rest.split_at_mut(sizes[1]);
    let (bk2, rest) = rest.split_at_mut(sizes[2]);
    let (mtau, rest) = rest.split_at_mut(sizes[3]);
    let (tau0, rest) = rest.split_at_mut(sizes[4]);
    let (mrho, rest) = rest.split_at_mut(sizes[5]);
    let (msigma, mlapl) = rest.split_at_mut(sizes[6]);

    // 1. The kinetic-energy functional.
    if !pol {
        let kin = MggaInput::new(rho, sigma, lapl, tau0, np, spin)?;
        eval_into(ked, &kin, order, bk1, d, np)?;
    } else {
        for ip in 0..np {
            mrho[2 * ip] = rho[2 * ip];
            msigma[3 * ip] = sigma[3 * ip];
            mlapl[2 * ip] = lapl[2 * ip];
        }
        {
            let kin = MggaInput::new(mrho, msigma, mlapl, tau0, np, spin)?;
            eval_into(ked, &kin, order, bk1, d, np)?;
        }
        for ip in 0..np {
            mrho[2 * ip] = rho[2 * ip + 1];
            msigma[3 * ip] = sigma[3 * ip + 2];
            mlapl[2 * ip] = lapl[2 * ip + 1];
        }
        let kin = MggaInput::new(mrho, msigma, mlapl, tau0, np, spin)?;
        eval_into(ked, &kin, order, bk2, d, np)?;
    }

    // 2. tau = rho * e_ked, per spin channel.
    {
        let k1 = mgga_scratch_view(bk1, d, np);
        let k2 = mgga_scratch_view(bk2, d, np);
        for ip in 0..np {
            if pol {
                mtau[2 * ip] = rho[2 * ip] * k1.zk[ip];
                mtau[2 * ip + 1] = rho[2 * ip + 1] * k2.zk[ip];
            } else {
                mtau[ip] = rho[ip] * k1.zk[ip];
            }
        }
    }

    // 3. The base meta-GGA at that tau.
    {
        let inp = MggaInput::new(rho, sigma, lapl, mtau, np, spin)?;
        eval_into(base, &inp, order, bm, d, np)?;
    }

    // 4. Combine.
    let m = mgga_scratch_view(bm, d, np);
    let k1 = mgga_scratch_view(bk1, d, np);
    let k2v = mgga_scratch_view(bk2, d, np);
    let k2 = if pol { &k2v } else { &k1 };
    if let Some(zk) = output.zk.as_deref_mut() {
        zk.copy_from_slice(&m.zk[..np]);
    }
    if order >= DerivativeOrder::Vxc {
        chain::order_1(np, pol, d, output, &m, &k1, k2);
    }
    if order >= DerivativeOrder::Fxc {
        chain::order_2(np, pol, d, output, &m, &k1, k2);
    }
    if order >= DerivativeOrder::Kxc {
        chain::order_3(np, pol, d, output, &m, &k1, k2);
    }
    if order >= DerivativeOrder::Lxc {
        chain::order_4(np, pol, d, output, &m, &k1, k2);
    }
    Ok(())
}

/// One auxiliary into a scratch block laid out like the workspace's.
fn eval_into(
    f: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    buf: &mut [f64],
    d: &Dimensions,
    np: usize,
) -> Result<(), LibxcRsError> {
    let mut out = mgga_scratch_output(mgga_scratch_view(buf, d, np), order);
    dispatch_mgga_by_id(f.meta.id, input, order, &mut out, f.kernel_ext_params(), &f.thresholds)
}

/// Every field of `order` and below present at its length; every supplied
/// field cleared.
fn check_and_zero(
    o: &mut MggaOutput<'_>,
    d: &Dimensions,
    np: usize,
    order: DerivativeOrder,
) -> Result<(), LibxcRsError> {
    let wanted = |rank: u8| rank <= order as u8;
    macro_rules! fields {
        ($($f:ident: $rank:literal),+ $(,)?) => {$(
            let expected = d.$f as usize * np;
            match o.$f.as_deref_mut() {
                Some(b) => {
                    if wanted($rank) && b.len() != expected {
                        return Err(LibxcRsError::OutputBufferSizeMismatch {
                            field: stringify!($f),
                            expected,
                            actual: b.len(),
                        });
                    }
                    b.fill(0.0);
                }
                None if wanted($rank) => {
                    return Err(LibxcRsError::OutputBufferSizeMismatch {
                        field: stringify!($f),
                        expected,
                        actual: 0,
                    });
                }
                None => {}
            }
        )+};
    }
    fields!(
        zk: 0, vrho: 1, vsigma: 1, vlapl: 1, vtau: 1, v2rho2: 2, v2rhosigma: 2, v2rholapl: 2,
        v2rhotau: 2, v2sigma2: 2, v2sigmalapl: 2, v2sigmatau: 2, v2lapl2: 2, v2lapltau: 2,
        v2tau2: 2, v3rho3: 3, v3rho2sigma: 3, v3rho2lapl: 3, v3rho2tau: 3, v3rhosigma2: 3,
        v3rhosigmalapl: 3, v3rhosigmatau: 3, v3rholapl2: 3, v3rholapltau: 3, v3rhotau2: 3,
        v3sigma3: 3, v3sigma2lapl: 3, v3sigma2tau: 3, v3sigmalapl2: 3, v3sigmalapltau: 3,
        v3sigmatau2: 3, v3lapl3: 3, v3lapl2tau: 3, v3lapltau2: 3, v3tau3: 3, v4rho4: 4,
        v4rho3sigma: 4, v4rho3lapl: 4, v4rho3tau: 4, v4rho2sigma2: 4, v4rho2sigmalapl: 4,
        v4rho2sigmatau: 4, v4rho2lapl2: 4, v4rho2lapltau: 4, v4rho2tau2: 4, v4rhosigma3: 4,
        v4rhosigma2lapl: 4, v4rhosigma2tau: 4, v4rhosigmalapl2: 4, v4rhosigmalapltau: 4,
        v4rhosigmatau2: 4, v4rholapl3: 4, v4rholapl2tau: 4, v4rholapltau2: 4, v4rhotau3: 4,
        v4sigma4: 4, v4sigma3lapl: 4, v4sigma3tau: 4, v4sigma2lapl2: 4, v4sigma2lapltau: 4,
        v4sigma2tau2: 4, v4sigmalapl3: 4, v4sigmalapl2tau: 4, v4sigmalapltau2: 4,
        v4sigmatau3: 4, v4lapl4: 4, v4lapl3tau: 4, v4lapl2tau2: 4, v4lapltau3: 4, v4tau4: 4,
    );
    Ok(())
}
