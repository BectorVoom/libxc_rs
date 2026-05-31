//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 982/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk982<F: Float>(t19451: F, t8533: F, t28002: F, t33231: F, t4028: F, t28864: F, t7042: F, t33222: F, t96797: F, t28952: F, t8526: F, t29219: F) -> (F, F, F, F, F, F, F) {
    let t127722 = F::cast_from(2.0_f64) * t19451 * t8533;
    let t127726 = F::cast_from(4.0_f64) * t28002 * t8533;
    let t127728 = F::cast_from(4.0_f64) * t4028 * t33231;
    let t127730 = F::cast_from(2.0_f64) * t7042 * t28864;
    let t127736 = F::cast_from(4.0_f64) * t96797 * t33222;
    let t127738 = F::cast_from(2.0_f64) * t8526 * t28952;
    let t127742 = F::cast_from(4.0_f64) * t8526 * t29219;
    (t127722, t127726, t127728, t127730, t127736, t127738, t127742)
}
