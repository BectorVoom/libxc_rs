//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 362/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk362<F: Float>(t7320: F, t271: F, t4765: F, t1303: F, t20: F, t2018: F, t1311: F, t1326: F, t2048: F) -> (F, F, F, F, F, F, F) {
    let t7321 = 1.0 / t7320;
    let t7322 = t7321 * t271;
    let t7323 = t4765 * t7322;
    let t7334 = t1303 * t20;
    let t7335 = t7334 * t2018;
    let t7344 = t1311 * t20;
    let t7345 = t7344 * t2018;
    let t7348 = t1326 * t2048;
    (t7321, t7323, t7334, t7335, t7344, t7345, t7348)
}
