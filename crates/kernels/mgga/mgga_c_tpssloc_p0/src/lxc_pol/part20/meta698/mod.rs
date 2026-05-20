//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta698 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2664;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta698<F: Float>(t157: F, t54349: F, t54372: F, t17: F, t184: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t54313: F, t54315: F, t54317: F, t54318: F, t54319: F, t54320: F, t54321: F, t54322: F, t54324: F, t54326: F, t39365: F, t15908: F, t9885: F, t9888: F, t39374: F, t39387: F, t15968: F, t172: F, t763: F, t5154: F, t9713: F, t39360: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t54374, t54376, t54377) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2664::<F>(t157, t54349, t54372, t17, t184, t39324, t39327, t39338, t39346, t39349, t39356, t54313, t54315, t54317, t54318, t54319, t54320, t54321, t54322, t54324, t54326);
        let (t54379, t54381, t54383, t54384, t54385, t54388, t54390, t54391) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2665::<F>(t39365, t15908, t9885, t9888, t39374, t39387, t15968, t172, t763, t5154, t9713, t39360, t39364, t39373, t39384, t39393, t39397, t39400, t39408, t39411);
    (t54374, t54376, t54377, t54379, t54381, t54383, t54384, t54385, t54388, t54390, t54391)
}
