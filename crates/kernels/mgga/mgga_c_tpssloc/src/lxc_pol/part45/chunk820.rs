//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 820/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk820<F: Float>(t268: F, t547: F, t6559: F, t225: F, t22942: F, t22643: F, t2752: F, t606: F, t1887: F, t23069: F, t229: F, t1902: F, t2678: F, t23226: F, t23228: F, t214: F, t2710: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t81228 = t6559 * t547 * t268;
    let t81319 = t22942 * t225;
    let t81326 = t22643 * t225;
    let t81547 = t2752 * t606;
    let t81591 = t23069 * t1887;
    let t81651 = t6559 * t229 * t268;
    let t82034 = t1902 * t2678;
    let t82071 = t23226 * t225;
    let t82074 = t23228 * t225;
    let t82124 = t214 * t2710;
    (t81228, t81319, t81326, t81547, t81591, t81651, t82034, t82071, t82074, t82124)
}
