//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1049/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1049<F: Float>(t21749: F, t4908: F, t18420: F, t4904: F, t20246: F, t338: F, t11556: F, t15300: F, t15364: F, t15376: F, t18447: F, t18452: F, t18455: F, t18458: F, t18460: F, t18489: F, t18530: F, t18533: F, t18536: F, t3447: F, t463: F, t4889: F, t6123: F, t6127: F, t6131: F) -> (F, F) {
    let t22090 = t4908 * t21749;
    let t22095 = t18420 * t4904;
    let t22104 = t20246 * t338;
    let t22112 = F::new(0.22222222222222222221e-2) * t4889 * t6131 + F::new(0.44444444444444444442e-2) * t4889 * t6127 - F::new(0.16666666666666666666e-2) * t3447 * t22090 - F::new(0.44444444444444444443e-2) * t15376 * t6123 + F::new(0.83333333333333333331e-3) * t3447 * t22095 + F::new(0.55555555555555555554e-3) * t18447 - F::new(0.55555555555555555554e-3) * t18452 - F::new(0.27777777777777777777e-3) * t18455 + F::new(0.37037037037037037036e-3) * t18458 + F::new(0.14814814814814814814e-2) * t18460 + F::new(0.18518518518518518518e-3) * t15300 - F::new(0.38024691358024691358e-1) * t22104 * t463 + F::new(0.55555555555555555554e-3) * t15364 + F::new(0.81481481481481481478e-2) * t18489 - F::new(0.83333333333333333331e-3) * t18530 - F::new(0.83333333333333333331e-3) * t18533 + F::new(0.44444444444444444443e-2) * t18536 + t11556;
    (t22104, t22112)
}
