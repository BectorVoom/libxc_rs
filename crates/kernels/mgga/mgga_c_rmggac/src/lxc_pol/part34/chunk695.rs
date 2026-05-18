//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 695/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk695<F: Float>(t2044: F, t25636: F, t2048: F, t3826: F, t328: F, t3810: F, t2566: F, t321: F, t1326: F, t13937: F, t13911: F, t333: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69195 = t25636 * t2044;
    let t69199 = t3826 * t2048;
    let t69200 = t69199 * t328;
    let t69201 = F::new(0.2419210303588817044e-2) * t69200;
    let t69205 = t3810 * t2048;
    let t69206 = t69205 * t2566;
    let t69211 = t2048 * t321;
    let t69212 = t1326 * t69211;
    let t69213 = t13937 * t69212;
    let t69234 = t13911 * t69212;
    let t69239 = t2048 * t333;
    (t69195, t69199, t69201, t69205, t69206, t69211, t69212, t69213, t69234, t69239)
}
