//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 681/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk681<F: Float>(t249: F, t2571: F, t2602: F, t2603: F, t2606: F, t2610: F, t2614: F, t2618: F, t2621: F, t2623: F, t2630: F, t2635: F, t2640: F, t2643: F, t2649: F, t2681: F, t2686: F, t2695: F, t2698: F, t2703: F, t2707: F, t787: F, t817: F, t831: F, t843: F, t849: F) -> F {
    let t2710 = t2602 + F::new(7.0) / F::new(72.0) * t2603 + t2571 * t2606 / F::new(16.0) - t787 * t2610 / F::new(48.0) + t2614 * t249 / F::new(3072.0) - t2618 * t831 / F::new(1536.0) - F::new(7.0) / F::new(2304.0) * t2621 - t2623 * t849 / F::new(384.0) + t2630 * t2635 / F::new(1536.0) + F::new(7.0) / F::new(2304.0) * t2640 + t2643 * t2649 / F::new(384.0) - t817 * t2681 / F::new(3072.0) - t817 * t2686 / F::new(3072.0) + t2695 + F::new(7.0) / F::new(576.0) * t2698 + F::new(5.0) / F::new(768.0) * t843 * t2703 - t843 * t2707 / F::new(768.0);
    t2710
}
