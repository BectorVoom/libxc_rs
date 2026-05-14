//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 783/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk783<F: Float>(t74806: F, t7782: F, t75516: F, t7788: F, t69463: F, t69465: F, t69469: F, t25820: F, t75836: F, t25877: F, t75839: F, t25854: F, t75842: F, t14327: F, t15093: F, t15078: F, t6444: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76379 = t7782 * t74806;
    let t76381 = t7788 * t75516;
    let t76414 = 0.15965655602485078085e0 * t69463;
    let t76415 = 0.15965655602485078085e0 * t69465;
    let t76416 = 0.23948483403727617128e0 * t69469;
    let t76425 = 0.17961362552795712846e0 * t25820 * t75836;
    let t76427 = 0.35922725105591425692e0 * t25877 * t75839;
    let t76429 = 0.17961362552795712846e0 * t25854 * t75842;
    let t76435 = t15093 * t14327;
    let t76440 = t6444 * t15078;
    (t76379, t76381, t76414, t76415, t76416, t76425, t76427, t76429, t76435, t76440)
}
