//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 837/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk837<F: Float>(t14011: F, t14047: F, t75027: F, t11654: F, t14236: F, t14249: F, t2078: F, t2812: F, t880: F, t1971: F, t14258: F, t14116: F, t14117: F, t8446: F) -> (F, F, F, F) {
    let t75029 = t14047 * t14011 * t75027;
    let t75033 = t14236 * t14249 * t2078 * t11654;
    let t75035 = t880 * t2812;
    let t75036 = t1971 * t75035;
    let t75037 = t14258 * t75036;
    let t75040 = t14116 * t14117 * t8446;
    (t75029, t75033, t75037, t75040)
}
