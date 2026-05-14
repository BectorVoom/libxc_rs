//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 622/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk622<F: Float>(t1322: F, t838: F, t874: F, t25525: F, t3065: F, t14327: F, t3814: F, t2566: F, t69184: F, t797: F, t68740: F, t14298: F, t2123: F, t7778: F, t305: F, t5148: F, t68684: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69428 = t838 * t874 * t1322;
    let t69433 = t25525 * t3065;
    let t69436 = t3814 * t14327;
    let t69437 = t69436 * t2566;
    let t69439 = t797 * t69184;
    let t69444 = t797 * t68740;
    let t69445 = t69444 * t14298;
    let t69452 = t7778 * t2123;
    let t69453 = t305 * t69452;
    let t69463 = t5148 * t68684;
    (t69428, t69433, t69436, t69437, t69439, t69444, t69445, t69452, t69453, t69463)
}
