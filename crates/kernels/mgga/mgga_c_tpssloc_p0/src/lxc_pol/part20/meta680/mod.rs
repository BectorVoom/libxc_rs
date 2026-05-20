//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2565;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta680<F: Float>(t11478: F, t4869: F, t11282: F, t1164: F, t14854: F, t4857: F, t14961: F, t3411: F, t11311: F, t1694: F, t44154: F, t11947: F, t3637: F, t4700: F, t5091: F, t51641: F, t51669: F, t51736: F, t51738: F, t51741: F, t51744: F, t14829: F, t3400: F, t4883: F, t14960: F, t3396: F, t15036: F, t11126: F, t4879: F, t11634: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t51870, t51874, t51880, t51884, t51885) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2565::<F>(t11478, t4869, t11282, t1164, t14854, t4857, t14961, t3411, t11311, t1694, t44154, t11947, t3637, t4700, t5091, t51641, t51669, t51736, t51738, t51741, t51744);
        let (t51889, t51892, t51898, t51903, t51905) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2566::<F>(t1164, t14829, t3400, t4883, t14960, t3396, t15036, t3411, t11126, t4879, t11634, t4869);
    (t51870, t51874, t51880, t51884, t51885, t51889, t51892, t51898, t51903, t51905)
}
