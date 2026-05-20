//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1704;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta366<F: Float>(t2363: F, t88: F, t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t1453: F, t2332: F, t9365: F, t2331: F, t4067: F, t666: F, t2358: F, t4043: F, t1444: F, t2342: F, t9384: F, t2341: F, t92: F, t2219: F, t659: F, t2248: F, t4049: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12739, t12747, t12750, t12752, t12754, t12757) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1704::<F>(t2363, t88, t1454, t2281, t4044, t626, t4068, t1453, t2332, t9365, t2331, t4067);
        let (t12758, t12761, t12771, t12774, t12775, t12778) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1705::<F>(t12757, t666, t2358, t4043, t1444, t2342, t9384, t2341, t92, t2219, t659, t2248, t4049);
    (t12739, t12747, t12750, t12752, t12754, t12757, t12758, t12761, t12771, t12774, t12775, t12778)
}
