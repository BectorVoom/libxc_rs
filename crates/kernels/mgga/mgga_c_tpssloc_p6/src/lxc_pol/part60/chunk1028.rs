//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1028/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1028<F: Float>(t33603: F, t7685: F, t1983: F, t28834: F, t31758: F, t191: F, t192: F, t29241: F, t2020: F, t127114: F, t2095: F, t115925: F, t28831: F) -> (F, F, F, F, F) {
    let t128564 = F::cast_from(6.0_f64) * t7685 * t33603;
    let t128567 = F::cast_from(3.0_f64) * t1983 * t31758 * t28834;
    let t128570 = t29241 * t191 * t192;
    let t128571 = t128570 * t2020;
    let t128573 = t1983 * t2095 * t127114;
    let t128575 = F::cast_from(6.0_f64) * t115925 * t28831;
    (t128564, t128567, t128571, t128573, t128575)
}
