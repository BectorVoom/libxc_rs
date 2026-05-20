//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2548/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2548<F: Float>(t21810: F, t3259: F, t50834: F, t51137: F, t63291: F, t63306: F, t63308: F, t63841: F, t63843: F, t63845: F, t71333: F, t71335: F, t71337: F) -> (F, F) {
    let t71547 = F::new(1.0) * t3259 * t21810;
    let t71558 = -F::cast_from(0.59793333333333333334e0_f64) * t63291 + F::cast_from(0.19931111111111111111e0_f64) * t63306 - F::cast_from(0.33218518518518518518e0_f64) * t63308 - F::cast_from(0.27385555555555555556e-1_f64) * t71333 + F::cast_from(0.54771111111111111112e-1_f64) * t71335 - F::cast_from(0.32862666666666666666e0_f64) * t71337 + t51137 - F::cast_from(0.93011851851851851854e0_f64) * t50834 - F::cast_from(0.73028148148148148146e-1_f64) * t63841 - F::cast_from(0.32862666666666666666e0_f64) * t63843 + F::cast_from(0.5477111111111111111e-1_f64) * t63845;
    (t71547, t71558)
}
