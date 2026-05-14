//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 240/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk240<F: Float>(t1022: F, t360: F, t1021: F, t248: F, t34: F, t365: F, t35: F, t364: F, t354: F, t122: F, t374: F, t376: F) -> (F, F, F, F, F, F) {
    let t1023 = t1022 * t360;
    let t1025 = t248 * t1021 * t1023;
    let t1028 = t365 * t34;
    let t1030 = 1.0 / t35 / t1028;
    let t1031 = t364 * t1030;
    let t1032 = t354 * t1031;
    let t1036 = t374 * t122 * t376;
    (t1023, t1025, t1030, t1031, t1032, t1036)
}
