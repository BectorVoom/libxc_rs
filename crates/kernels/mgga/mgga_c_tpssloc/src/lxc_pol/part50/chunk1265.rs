//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1265/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1265<F: Float>(t31193: F, t5187: F, t6637: F, t6888: F, t114064: F, t120441: F, t120445: F, t120447: F, t120452: F, t120456: F, t120459: F, t120463: F, t120467: F, t120468: F, t120469: F, t120471: F, t120475: F, t120483: F, t1336: F, t1352: F, t1814: F, t31211: F, t31212: F, t31214: F, t5230: F, t5234: F, t5287: F, t8483: F) -> F {
    let t120487 = F::new(0.3289868133696452873e-1) * t6888 * t6637 * t31193 * t5187;
    let t120488 = -t120475 * t1336 * t1352 - t1336 * t31211 * t5287 + t1814 * t31214 - t31212 * t5234 + t5230 * t8483 - t114064 + t120441 - t120445 + t120447 - t120452 - t120456 + t120459 + t120463 + t120467 + t120468 + t120469 + t120471 - t120483 - t120487;
    t120488
}
