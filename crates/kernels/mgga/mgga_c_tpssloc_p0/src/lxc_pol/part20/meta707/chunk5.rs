//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2703/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2703<F: Float>(t16486: F, t3701: F, t1388: F, t3914: F, t15899: F, t16148: F, t16497: F, t3719: F, t3918: F, t3919: F, t39338: F, t39346: F, t39349: F, t39356: F, t5126: F, t5160: F, t54321: F, t54322: F, t54324: F) -> F {
    let t55169 = t16486 * t3701;
    let t55173 = t3914 * t1388;
    let t55180 = -F::new(3.0) * t1388 * t5160 * t55169 + F::new(6.0) * t15899 * t5160 * t55173 + F::new(36.0) * t16148 * t3919 * t5126 + F::new(9.0) * t16497 * t3719 * t3918 - t39338 + t39346 + t39349 + t39356 + t54321 - t54322 + t54324;
    t55180
}
