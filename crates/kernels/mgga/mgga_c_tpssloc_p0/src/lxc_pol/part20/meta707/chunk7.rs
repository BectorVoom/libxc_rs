//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2705/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2705<F: Float>(t12156: F, t12303: F, t12477: F, t1390: F, t16153: F, t16490: F, t1845: F, t193: F, t3918: F, t3919: F, t39483: F, t5122: F, t5126: F, t5187: F, t54404: F, t54406: F, t54409: F, t54411: F, t54413: F) -> F {
    let t55217 = F::new(6.0) * t12156 * t1390 * t1845 * t193 + F::new(18.0) * t12303 * t5122 * t5126 - F::new(9.0) * t12477 * t3918 * t5187 + F::new(18.0) * t16153 * t3919 * t5126 + F::new(18.0) * t16490 * t193 * t5187 + t39483 - t54404 - t54406 + t54409 + t54411 - t54413;
    t55217
}
