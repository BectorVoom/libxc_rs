//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 832/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk832<F: Float>(t1041: F, t14202: F, t1615: F, t3131: F, t360: F, t883: F, t1573: F, t2904: F, t1561: F, t2885: F, t2860: F, t2929: F, t1603: F, t3030: F, t3032: F, t3129: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14203 = t1041 * t14202;
    let t14211 = t1615 * t3131;
    let t14219 = t360 * t883;
    let t14263 = t1573 * t2904;
    let t14271 = t1561 * t2885;
    let t14276 = t1561 * t2860;
    let t14337 = t1573 * t2929;
    let t14506 = t1603 * t3030;
    let t14507 = t14506 * t3032;
    let t14508 = t14507 * t3129;
    (t14203, t14211, t14219, t14263, t14271, t14276, t14337, t14506, t14507, t14508)
}
