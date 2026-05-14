//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 841/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk841<F: Float>(t336: F, t363: F, t5600: F, t5604: F, t5605: F, t5610: F, t5614: F, t5618: F, t5620: F, t931: F, t951: F, t972: F) -> (F,) {
    let t5623 = -t5600 * t336 / 36.0 + t5604 + t5605 * t931 / 288.0 + t5610 * t951 / 1536.0 - t5614 * t363 / 288.0 + t5618 + t5620 * t972 / 2304.0;
    (t5623,)
}
