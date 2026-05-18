//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 482/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk482<F: Float>(t13862: F, t323: F, t3133: F, t3046: F, t6444: F, t333: F, t3851: F, t2048: F, t793: F, t328: F, t3814: F, t2566: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13892 = t13862 * t323;
    let t13893 = t3133 * t13892;
    let t13895 = t6444 * t3046;
    let t13897 = t3046 * t333;
    let t13898 = t3851 * t13897;
    let t13900 = t793 * t2048;
    let t13902 = t3851 * t3046;
    let t13903 = t13902 * t328;
    let t13905 = t3814 * t3046;
    let t13906 = t13905 * t2566;
    (t13892, t13893, t13895, t13897, t13898, t13900, t13902, t13903, t13905, t13906)
}
