//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 36/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk36<F: Float>(t36: F, t25: F, t48: F, t28: F, rho0: F, rho1: F, tau0: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t91 = F::cast_from(1.0_f64) / t36 / rho0;
    let t92 = tau0 * t91;
    let t93 = t25 / F::cast_from(2.0_f64);
    let t94 = pow_1_3::<F>(t93);
    let t95 = t94 * t94;
    let t96 = t95 * t93;
    let t99 = F::cast_from(1.0_f64) / t48 / rho1;
    let t100 = tau1 * t99;
    let t101 = t28 / F::cast_from(2.0_f64);
    let t102 = pow_1_3::<F>(t101);
    let t103 = t102 * t102;
    let t104 = t103 * t101;
    let t106 = t100 * t104 + t92 * t96;
    let t107 = F::cast_from(1.0_f64) / t106;
    (t92, t94, t95, t96, t100, t102, t103, t104, t106, t107)
}
