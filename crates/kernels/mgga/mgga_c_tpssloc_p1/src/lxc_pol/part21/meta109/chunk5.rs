//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 757/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk757<F: Float>(t249: F, t2571: F, t2602: F, t2603: F, t2606: F, t2610: F, t2614: F, t2618: F, t2621: F, t2623: F, t2630: F, t2635: F, t2640: F, t2643: F, t2649: F, t2681: F, t2686: F, t2695: F, t2698: F, t2703: F, t2707: F, t787: F, t817: F, t831: F, t843: F, t849: F) -> F {
    let t2710 = t2602 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2603 + t2571 * t2606 / F::cast_from(16.0_f64) - t787 * t2610 / F::cast_from(48.0_f64) + t2614 * t249 / F::cast_from(3072.0_f64) - t2618 * t831 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t2621 - t2623 * t849 / F::cast_from(384.0_f64) + t2630 * t2635 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t2640 + t2643 * t2649 / F::cast_from(384.0_f64) - t817 * t2681 / F::cast_from(3072.0_f64) - t817 * t2686 / F::cast_from(3072.0_f64) + t2695 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t2698 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t843 * t2703 - t843 * t2707 / F::cast_from(768.0_f64);
    t2710
}
