//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1150;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta354<F: Float>(t207: F, t40394: F, t40399: F, t786: F, t9580: F, t2566: F, t2570: F, t2588: F, t40341: F, t215: F, t39933: F, t40344: F, t795: F, t116: F, t9534: F, t39568: F, t761: F, t39382: F, t39302: F, t6589: F, t68: F, t236: F, t40931: F, t240: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41185, t41189, t41196, t41200, t41209, t41212) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1150::<F>(t207, t40394, t40399, t786, t9580, t2566, t2570, t2588, t40341, t215, t39933, t40344, t795);
        let (t41214, t41254, t41258, t41262, t41315, t41349) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1151::<F>(t116, t786, t9534, t39568, t761, t39382, t39302, t6589, t68, t236, t40931, t240, t812);
    (t41185, t41189, t41196, t41200, t41209, t41212, t41214, t41254, t41258, t41262, t41315, t41349)
}
