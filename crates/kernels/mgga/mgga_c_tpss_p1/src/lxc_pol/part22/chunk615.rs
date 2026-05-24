//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 615/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk615<F: Float>(t2751: F, t837: F, t242: F, t967: F, t2469: F, t970: F, t277: F, t836: F) -> (F, F, F, F) {
    let t2752 = t2751 * t837;
    let t2753 = t242 * t2752;
    let t2754 = t967 * t2753;
    let t2756 = t970 * t2469;
    let t2757 = t242 * t2756;
    let t2761 = F::new(1.0) / t277 / t836;
    (t2753, t2754, t2757, t2761)
}
