//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1253/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1253<F: Float>(t19112: F, t5570: F, t1883: F, t5637: F, t9738: F, t19128: F, t6021: F, t19077: F, t9546: F, t8550: F, t9605: F, t9615: F, t19075: F, t1872: F, t9533: F, t6013: F, t9525: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t63208 = t19112 * t5570;
    let t63219 = t1883 * t5637 * t9738;
    let t63237 = t6021 * t19128;
    let t63250 = t19077 * t9546;
    let t63254 = t8550 * t9615 * sigma2 * t9605;
    let t63258 = t8550 * t19075 * t9605;
    let t63268 = 5.0 / 1296.0 * t1872 * t9533;
    let t63269 = t6013 * t9525;
    (t63208, t63219, t63237, t63250, t63254, t63258, t63268, t63269)
}
