//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2704/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2704<F: Float>(t1307: F, t3698: F, t1390: F, t16486: F, t16497: F, t3734: F, t3918: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t5126: F, t54385: F, t54388: F, t54390: F) -> (F, F) {
    let t55183 = t3698 * t1307;
    let t55191 = t16486 * t1390;
    let t55195 = F::new(9.0) * t1307 * t3918 * t55191 + F::new(18.0) * t16497 * t3734 * t5126 - t39384 + t39393 - t39397 - t39400 + t39408 + t39411 - t54385 - t54388 - t54390;
    (t55183, t55195)
}
