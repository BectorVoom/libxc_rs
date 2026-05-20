//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta557<F: Float>(t82122: F, t82153: F, t82218: F, t81440: F, t1453: F, t81439: F, t26129: F, t81442: F, t22470: F, t4067: F, t25: F, t40772: F) -> (F, F, F, F, F, F, F, F) {
        let (t85060, t85101, t85129, t86583, t86586, t86588, t86590, t86716) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1785::<F>(t82122, t82153, t82218, t81440, t1453, t81439, t26129, t81442, t22470, t4067, t25, t40772);
    (t85060, t85101, t85129, t86583, t86586, t86588, t86590, t86716)
}
