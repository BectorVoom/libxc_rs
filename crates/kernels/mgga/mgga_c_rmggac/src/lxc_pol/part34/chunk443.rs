//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 443/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk443<F: Float>(t3046: F, t3851: F, t328: F, t3814: F, t2566: F, t2048: F, t637: F, t797: F, t1322: F) -> (F, F, F, F, F, F) {
    let t13902 = t3851 * t3046;
    let t13903 = t13902 * t328;
    let t13905 = t3814 * t3046;
    let t13906 = t13905 * t2566;
    let t13909 = t797 * t2048 * t637;
    let t13911 = t3814 * t1322;
    (t13902, t13903, t13905, t13906, t13909, t13911)
}
