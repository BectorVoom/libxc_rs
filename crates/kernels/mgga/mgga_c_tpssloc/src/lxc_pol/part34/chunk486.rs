//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 486/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk486<F: Float>(t172: F, t1787: F, t763: F, t67: F, t758: F, t193: F, t533: F, t1845: F, t3701: F, t750: F, t17: F, t1408: F, t3704: F, t1649: F, t3711: F, t1804: F, t3726: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    let t5160 = t193 * t533;
    let t5161 = t1845 * t3701;
    let t5168 = t1787 * t750;
    let t5169 = t17 * t5168;
    let t5170 = t3704 * t1408;
    let t5178 = t3711 * t1649;
    let t5192 = t3726 * t1804;
    (t5154, t5155, t5157, t5158, t5160, t5161, t5168, t5169, t5170, t5178, t5192)
}
