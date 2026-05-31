//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 769/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk769<F: Float>(t232: F, t9957: F, t819: F, t820: F, t2571: F, t2618: F, t2643: F, t2649: F, t2686: F, t817: F, t9642: F, t9649: F, t9653: F, t9657: F, t9663: F, t9668: F, t9672: F, t9675: F, t9679: F) -> (F, F, F) {
    let t9958 = t9957 * t232;
    let t9960 = t819 * t820 * t9958;
    let t9963 = t9642 * t2649 / F::cast_from(128.0_f64) - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2643 * t9649 + t2643 * t9653 / F::cast_from(256.0_f64) + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2571 * t9657 - t817 * t9663 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t9668 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t9672 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t9675 - t2618 * t2686 / F::cast_from(1024.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t9679 - t817 * t9960 / F::cast_from(3072.0_f64);
    (t9958, t9960, t9963)
}
