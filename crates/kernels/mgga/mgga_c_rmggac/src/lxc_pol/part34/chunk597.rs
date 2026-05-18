//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 597/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk597<F: Float>(t3076: F, t570: F, t2044: F, t13839: F, t234: F, t551: F, t3157: F, t3167: F, t8368: F, t2367: F, t649: F, t27: F) -> (F, F, F, F, F, F) {
    let t15271 = t3076 * t570;
    let t15272 = t2044 * t15271;
    let t15273 = t13839 * t15272;
    let t15280 = t234 * t551;
    let t15281 = t15280 * t3157;
    let t15284 = t8368 * t3167;
    let t15286 = t649 * t2367;
    let t15287 = t27 * t15286;
    (t15272, t15273, t15280, t15281, t15284, t15287)
}
