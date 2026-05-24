//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 916/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk916<F: Float>(t2737: F, t8507: F, t2798: F, t2782: F, t2762: F, t774: F, t126: F, t2761: F, t242: F, t2460: F, t967: F, t2464: F, t277: F) -> (F, F, F, F, F) {
    let t8508 = t2737 * t8507;
    let t8509 = t2798 * t8508;
    let t8514 = t2782 * t8508;
    let t8523 = t774 * t2762;
    let t8528 = t126 * t2761;
    let t8530 = t242 * t8528 * t2460;
    let t8531 = t967 * t8530;
    let t8539 = F::new(1.0) / t277 / t2464;
    (t8509, t8514, t8523, t8531, t8539)
}
