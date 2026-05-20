//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2664/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2664<F: Float>(t157: F, t54349: F, t54372: F, t17: F, t184: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t54313: F, t54315: F, t54317: F, t54318: F, t54319: F, t54320: F, t54321: F, t54322: F, t54324: F, t54326: F) -> (F, F, F) {
    let t54374 = (t54349 + t54372) * t157;
    let t54376 = t17 * t54374 * t184;
    let t54377 = -t39324 + t54313 - t54315 - t54317 + t39327 + t54318 + t54319 - t54320 - t39338 + t54321 - t54322 + t39346 + t39349 + t54324 + t39356 - t54326 + t54376;
    (t54374, t54376, t54377)
}
