//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1159/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1159<F: Float>(t10179: F, t522: F, t1768: F, t3366: F, t3364: F, t5570: F, t1771: F) -> (F, F, F, F) {
    let t18490 = t10179 * t522;
    let t18492 = t18490 * t1768 * t3366;
    let t18495 = t5570 * t3364;
    let t18496 = t1771 * t18495;
    (t18490, t18492, t18495, t18496)
}
