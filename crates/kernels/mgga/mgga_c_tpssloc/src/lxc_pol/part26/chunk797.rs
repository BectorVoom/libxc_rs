//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 797/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk797<F: Float>(t232: F, t9957: F, t819: F, t820: F, t2571: F, t2618: F, t2643: F, t2649: F, t2686: F, t817: F, t9642: F, t9649: F, t9653: F, t9657: F, t9663: F, t9668: F, t9672: F, t9675: F, t9679: F) -> (F, F, F) {
    let t9958 = t9957 * t232;
    let t9960 = t819 * t820 * t9958;
    let t9963 = t9642 * t2649 / 128.0 - 5.0 / 256.0 * t2643 * t9649 + t2643 * t9653 / 256.0 + 3.0 / 16.0 * t2571 * t9657 - t817 * t9663 / 3072.0 - 7.0 / 768.0 * t9668 - 119.0 / 4608.0 * t9672 + 7.0 / 768.0 * t9675 - t2618 * t2686 / 1024.0 + 7.0 / 1536.0 * t9679 - t817 * t9960 / 3072.0;
    (t9958, t9960, t9963)
}
