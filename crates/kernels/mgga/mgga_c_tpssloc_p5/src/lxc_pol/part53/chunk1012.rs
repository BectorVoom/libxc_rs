//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1012/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1012<F: Float>(t10109: F, t8740: F, t114613: F, t114615: F, t121336: F, t121339: F, t121352: F, t121362: F, t121367: F, t121371: F, t13053: F, t13065: F, t13463: F, t24305: F, t25168: F, t2597: F, t2713: F, t32018: F, t33935: F, t33951: F, t4147: F, t4272: F, t7830: F, t8741: F) -> F {
    let t123464 = t10109 * t8740;
    let t123476 = -t13053 * t8741 - t13065 * t8741 + F::cast_from(0.6579736267392905746e-1_f64) * t121336 + F::cast_from(0.19739208802178717238e0_f64) * t121339 - t13463 * t8741 + F::cast_from(0.6579736267392905746e-1_f64) * t121352 - F::cast_from(0.3289868133696452873e-1_f64) * t114613 - F::cast_from(0.76763589786250567037e-1_f64) * t114615 - F::cast_from(0.19739208802178717238e0_f64) * t121362 + F::new(4.0) * t24305 * t7830 - F::new(6.0) * t25168 * t123464 * t4272 + F::cast_from(0.6579736267392905746e-1_f64) * t121367 - F::new(6.0) * t2713 * t33951 - F::new(6.0) * t4147 * t32018 - F::cast_from(0.15352717957250113407e0_f64) * t121371 + F::new(4.0) * t2597 * t33935;
    t123476
}
