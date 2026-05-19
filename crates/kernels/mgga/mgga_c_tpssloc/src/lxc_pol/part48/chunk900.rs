//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 900/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk900<F: Float>(t112673: F, t23030: F, t30660: F, t23204: F, t30656: F, t6562: F, t30624: F, t81591: F, t23270: F, t2379: F, t25038: F, t30622: F) -> (F, F, F, F, F) {
    let t112674 = F::cast_from(0.76763589786250567036e-1_f64) * t112673;
    let t112676 = F::cast_from(0.52089578783527170489e-1_f64) * t23030 * t30660;
    let t112678 = t6562 * t23204 * t30656;
    let t112679 = F::cast_from(0.16449340668482264365e-1_f64) * t112678;
    let t112680 = t81591 * t30624;
    let t112681 = F::cast_from(0.15352717957250113407e0_f64) * t112680;
    let t112685 = F::cast_from(0.9869604401089358619e-1_f64) * t25038 * t23270 * t30622 * t2379;
    (t112674, t112676, t112679, t112681, t112685)
}
