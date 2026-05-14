//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 500/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk500<F: Float>(t249: F, t2571: F, t2602: F, t2603: F, t2606: F, t2610: F, t2614: F, t2618: F, t2621: F, t2623: F, t2630: F, t2635: F, t2640: F, t2643: F, t2649: F, t2681: F, t2686: F, t2695: F, t2698: F, t2703: F, t2707: F, t787: F, t817: F, t831: F, t843: F, t849: F) -> (F,) {
    let t2710 = t2602 + 7.0 / 72.0 * t2603 + t2571 * t2606 / 16.0 - t787 * t2610 / 48.0 + t2614 * t249 / 3072.0 - t2618 * t831 / 1536.0 - 7.0 / 2304.0 * t2621 - t2623 * t849 / 384.0 + t2630 * t2635 / 1536.0 + 7.0 / 2304.0 * t2640 + t2643 * t2649 / 384.0 - t817 * t2681 / 3072.0 - t817 * t2686 / 3072.0 + t2695 + 7.0 / 576.0 * t2698 + 5.0 / 768.0 * t843 * t2703 - t843 * t2707 / 768.0;
    (t2710,)
}
