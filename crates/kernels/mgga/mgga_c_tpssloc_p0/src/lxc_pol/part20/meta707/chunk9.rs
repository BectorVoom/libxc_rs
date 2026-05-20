//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2707/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2707<F: Float>(t12461: F, t5356: F, t1388: F, t3719: F, t19577: F, t22578: F, t3698: F, t3918: F, t39367: F, t39585: F, t39590: F, t39593: F, t39595: F, t5160: F, t5161: F, t54433: F, t54435: F, t54436: F) -> F {
    let t55242 = t5356 * t12461;
    let t55246 = t1388 * t3719;
    let t55256 = -F::new(9.0) * t19577 * t22578 * t3918 + F::new(6.0) * t3698 * t5160 * t55242 - F::new(9.0) * t3918 * t39367 * t5161 - F::new(9.0) * t3918 * t5161 * t55246 - t39585 + t39590 - t39593 + t39595 + t54433 - t54435 + t54436;
    t55256
}
