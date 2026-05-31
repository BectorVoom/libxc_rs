//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1015/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1015<F: Float>(t114827: F, t114882: F, t116557: F, t116578: F, t121467: F, t121469: F, t1527: F, t25168: F, t259: F, t26702: F, t26728: F, t2713: F, t2718: F, t31964: F, t31998: F, t32006: F, t33935: F, t33947: F, t4268: F, t4273: F, t4300: F, t4301: F, t7106: F, t7841: F, t798: F, t855: F, t8740: F) -> F {
    let t123552 = -F::cast_from(0.3289868133696452873e-1_f64) * t121467 + F::cast_from(0.15352717957250113407e0_f64) * t121469 - F::cast_from(0.16449340668482264365e-1_f64) * t114827 - t31964 * t4301 + F::cast_from(2.0_f64) * t31964 * t4273 + t798 * t33947 * t259 + F::cast_from(4.0_f64) * t2713 * t33935 - F::cast_from(12.0_f64) * t25168 * t26728 * t26702 - t116557 + F::cast_from(2.0_f64) * t855 * t2718 * t8740 * t4300 + F::cast_from(0.76763589786250567037e-1_f64) * t114882 + F::cast_from(4.0_f64) * t855 * t2718 * t7106 * t7841 + t116578 + F::cast_from(2.0_f64) * t4268 * t32006 + F::cast_from(2.0_f64) * t855 * t2718 * t31998 * t1527;
    t123552
}
