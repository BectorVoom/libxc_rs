//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1562;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta403<F: Float>(t22473: F, t2332: F, t2358: F, t6530: F, t2303: F, t71: F, t33: F, t9228: F, t2235: F, t608: F, t641: F, t645: F, t72: F, t2307: F, t79: F, t2244: F, t605: F, t2251: F, t2241: F, t2240: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22474, t22476, t22489, t22493, t22519, t22527) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1562::<F>(t22473, t2332, t2358, t6530, t2303, t71, t33, t9228, t2235, t608, t641, t645, t72);
        let (t22531, t22534, t22537, t22546, t22549) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1563::<F>(t2307, t79, t72, t2244, t605, t2251, t2241, t2240, t608);
    (t22474, t22476, t22489, t22493, t22519, t22527, t22531, t22534, t22537, t22546, t22549)
}
