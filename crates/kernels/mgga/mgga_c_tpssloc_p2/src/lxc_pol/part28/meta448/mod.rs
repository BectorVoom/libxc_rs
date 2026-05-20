//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1637;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1638;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta448<F: Float>(t23967: F, t6492: F, t2031: F, t22550: F, t6495: F, t7032: F, t7025: F, t9231: F, t6486: F, t240: F, t67: F, t1864: F, t1860: F, t6509: F, t7031: F, t22489: F, t2032: F, t22493: F, t22519: F, t22527: F, t22531: F, t22534: F, t22537: F, t22546: F, t22549: F, t23963: F, t7026: F, t7035: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23968, t23970, t23973, t23975, t23978, t23992, t23993) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1637::<F>(t23967, t6492, t2031, t22550, t6495, t7032, t7025, t9231, t6486, t240, t67, t1864);
        let (t23995, t23998) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1638::<F>(t1860, t23993, t6509, t7031);
        let (t23999, t24001, t24006) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1639::<F>(t1860, t23998, t2031, t22489, t2032, t22493, t22519, t22527, t22531, t22534, t22537, t22546, t22549, t23963, t23968, t23970, t23973, t23975, t23978, t23995, t6486, t6492, t6495, t7026, t7035);
    (t23968, t23970, t23973, t23975, t23978, t23992, t23993, t23995, t23998, t23999, t24001, t24006)
}
