//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 261/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk261<F: Float>(t225: F, t382: F, t386: F, t68: F, t1011: F, t1014: F, t1010: F, t357: F, t360: F) -> (F, F, F, F, F, F) {
    let t1052 = t382 * t225;
    let t1053 = t386 * t386;
    let t1054 = F::cast_from(1.0_f64) / t1053;
    let t1055 = t68 * t1054;
    let t1057 = t1011 * t1014;
    let t1058 = t1010 * t1057;
    let t1060 = t357 * t360;
    (t1052, t1053, t1055, t1057, t1058, t1060)
}
