//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 366/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk366<F: Float>(t128: F, t209: F, t476: F, t118: F, t2106: F, t261: F, t1297: F, t20: F, t2018: F, t511: F, t892: F, t504: F, t880: F, t2144: F) -> (F, F, F, F, F, F, F, F) {
    let t7476 = t128 * t476 * t209;
    let t7477 = t118 * t7476;
    let t7487 = t261 * t2106;
    let t7490 = t1297 * t20;
    let t7491 = t7490 * t2018;
    let t7494 = t892 * t511;
    let t7501 = t504 * t880;
    let t7508 = t504 * t2144;
    (t7476, t7477, t7487, t7490, t7491, t7494, t7501, t7508)
}
