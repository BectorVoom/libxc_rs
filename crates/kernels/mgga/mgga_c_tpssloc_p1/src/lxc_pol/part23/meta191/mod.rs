//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta191<F: Float>(t1014: F, t10471: F, t10470: F, t360: F, t6739: F, t10474: F, t10482: F, t3127: F, t3131: F, t3215: F, t390: F, t268: F, t405: F, t6546: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11045, t11046, t11048, t11058, t11059, t11060, t11064, t11065, t11066, t11094, t11135) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk826::<F>(t1014, t10471, t10470, t360, t6739, t10474, t10482, t3127, t3131, t3215, t390, t268, t405, t6546);
    (t11045, t11046, t11048, t11058, t11059, t11060, t11064, t11065, t11066, t11094, t11135)
}
