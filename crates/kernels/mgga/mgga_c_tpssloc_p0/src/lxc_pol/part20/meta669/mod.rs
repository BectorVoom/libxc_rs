//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2515;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta669<F: Float>(t11243: F, t3271: F, t4756: F, t1102: F, t14758: F, t3270: F, t3287: F, t51000: F, t51004: F, t51007: F, t51010: F, t51012: F, t51014: F, t51016: F, t51018: F, t51021: F, t44938: F, t45971: F, t48140: F, t45192: F, t2403: F, t4775: F, t14795: F, t699: F, t14798: F, t136: F, t3297: F, t50959: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51024, t51027, t51030, t51032) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2515::<F>(t11243, t3271, t4756, t1102, t14758, t3270, t3287, t51000, t51004, t51007, t51010, t51012, t51014, t51016, t51018, t51021);
        let (t51034, t51037, t51039, t51040, t51041, t51043, t51046) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2516::<F>(t44938, t45971, t48140, t45192, t2403, t4775, t14795, t699, t14798, t136, t3297, t50959);
    (t51024, t51027, t51030, t51032, t51034, t51037, t51039, t51040, t51041, t51043, t51046)
}
