//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta800 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta800<F: Float>(t118: F, t2375: F, t5522: F, t46335: F, t46348: F, t16575: F, t706: F, t708: F, t46369: F, t46371: F, t39549: F, t39563: F, t39585: F, t39590: F, t39593: F, t40801: F, t40803: F, t58060: F, t58061: F, t58062: F, t58080: F, t58085: F, t58094: F) -> (F, F, F, F, F, F, F) {
        let (t58973, t58974, t58975, t58978, t58979, t58980, t58981) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2787::<F>(t118, t2375, t5522, t46335, t46348, t16575, t706, t708, t46369, t46371, t39549, t39563, t39585, t39590, t39593, t40801, t40803, t58060, t58061, t58062, t58080, t58085, t58094);
    (t58973, t58974, t58975, t58978, t58979, t58980, t58981)
}
