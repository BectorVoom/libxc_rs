//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1024/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1024<F: Float>(t11238: F, t11435: F, t345: F, t242: F, t947: F, t8431: F, t8435: F, t8439: F, t8453: F, t8456: F, t8462: F, t8472: F, t8481: F, t8484: F, t8500: F, t946: F) -> (F, F) {
    let t11436 = t11238 + t11435;
    let t11437 = t11436 * t345;
    let t11439 = t242 * t947 * t11437;
    let t11452 = t946 * t11439 / 3072.0 + t8431 / 4608.0 + t8435 / 2304.0 - t8439 / 4608.0 - t8453 / 162.0 - t8456 / 648.0 - t8462 / 648.0 - t8472 / 6912.0 + t8481 / 6912.0 + t8484 / 648.0 - t8500 / 432.0;
    (t11436, t11452)
}
