//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2712/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2712<F: Float>(t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t56167: F, t56169: F, t56170: F, t56171: F, t56172: F, t56173: F, t56178: F, t56179: F, t56186: F) -> F {
    let t57196 = t39364 - t56167 + t56169 + t56170 + t56171 + t39373 - t56172 - t39384 - t56173 + t39393 - t39397 - t39400 + t39408 + t39411 - t56178 - t56179 - t56186;
    t57196
}
