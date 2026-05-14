//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 801/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk801<F: Float>(t22751: F, t6970: F, t3777: F, t6944: F, t3787: F, t59: F, t6943: F, t835: F, t1336: F, t1354: F, t6604: F, t6919: F, t6937: F, t6950: F, t1369: F, t6951: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22752 = t22751 * t6970;
    let t22756 = t3777 * t6944;
    let t22759 = t3787 * t59;
    let t22764 = t6943 * t835;
    let t22765 = t1336 * t22764;
    let t22766 = t22765 * t1354;
    let t22779 = t6919 * t6604;
    let t22780 = t22779 * t6937;
    let t22782 = t6950 * t835;
    let t22783 = t1336 * t22782;
    let t22784 = t22783 * t1369;
    let t22788 = t3777 * t6951;
    (t22752, t22756, t22759, t22765, t22766, t22779, t22780, t22783, t22784, t22788)
}
