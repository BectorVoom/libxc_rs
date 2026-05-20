//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2708/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2708<F: Float>(t193: F, t3734: F, t1845: F, t40611: F, t12458: F, t1307: F, t15868: F, t15883: F, t15904: F, t16018: F, t3719: F, t3918: F, t39639: F, t5126: F, t5131: F, t5160: F, t54447: F, t54448: F, t54449: F, t54450: F, t54452: F, t571: F) -> F {
    let t55266 = t193 * t3734;
    let t55276 = t1845 * t40611;
    let t55280 = F::new(18.0) * t1307 * t16018 * t5126 * t571 - F::new(6.0) * t12458 * t5160 * t55276 - F::new(18.0) * t15868 * t15904 * t3918 + F::new(18.0) * t15883 * t3719 * t5126 + F::new(18.0) * t5131 * t55266 + t39639 - t54447 - t54448 + t54449 - t54450 + t54452;
    t55280
}
