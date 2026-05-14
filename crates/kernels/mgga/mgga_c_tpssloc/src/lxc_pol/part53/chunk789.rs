//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 789/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk789<F: Float>(t31320: F, t798: F, t8728: F, t30697: F, t30704: F, t30721: F, t30701: F, t30707: F, t30710: F, t30717: F, t30723: F, t218: F, t31374: F, t31382: F, t814: F, t829: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31971 = 0.16449340668482264365e-1 * t31320;
    let t31974 = t798 * t8728;
    let t31976 = 0.22608743412718618877e-1 * t30697;
    let t31978 = 0.5383034145885385447e-3 * t30704;
    let t31982 = 7.0 / 576.0 * t30721;
    let t31984 = -t31976 - 0.19378922925187387609e-1 * t30701 - t31978 - 0.32298204875312312682e-2 * t30707 + t30710 / 384.0 - t30717 / 384.0 - t31982 - t30723 / 96.0;
    let t31985 = t218 * t31984;
    let t31987 = 0.76763589786250567037e-1 * t31374;
    let t31989 = 0.16449340668482264365e-1 * t31382;
    let t31993 = t814 * t8728;
    let t31994 = t31993 * t829;
    (t31971, t31974, t31976, t31978, t31982, t31984, t31985, t31987, t31989, t31993, t31994)
}
