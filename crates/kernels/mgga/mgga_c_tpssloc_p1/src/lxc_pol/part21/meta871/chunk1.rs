//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3201/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3201<F: Float>(t15569: F, t15572: F, t11665: F, t15714: F, t15740: F, t15749: F, t18364: F, t3242: F, t3508: F, t3577: F, t3578: F, t45250: F, t4733: F, t4950: F, t5012: F, t52615: F, t53433: F, t53440: F, t53452: F, t66372: F, t66378: F, t66380: F, t66566: F, t66571: F, t66575: F, t66583: F, t66597: F) -> F {
    let t66599 = t15569 * t15572;
    let t66601 = -t66566 / F::new(3456.0) + F::new(5.0) / F::new(6912.0) * t11665 * t18364 - t45250 + t66571 / F::new(324.0) - F::new(2.0) / F::new(243.0) * t53433 + t66575 / F::new(162.0) - F::new(5.0) / F::new(1944.0) * t53440 + F::new(5.0) / F::new(6912.0) * t15740 * t15714 + t52615 * t4950 / F::new(216.0) + F::new(5.0) / F::new(1728.0) * t66378 * t66583 * t3508 * t3242 * t66380 - F::new(5.0) / F::new(3456.0) * t66372 * t66583 * t15749 - t53452 / F::new(1728.0) - t3577 * t3578 * t5012 * t4733 / F::new(1152.0) - t66597 / F::new(1728.0) + t66599 / F::new(324.0);
    t66601
}
