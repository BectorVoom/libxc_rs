//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1126;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1127;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1128;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta343<F: Float>(t39273: F, t39275: F, t39278: F, t39281: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t683: F, t702: F, t39378: F, t746: F, t9720: F, t1294: F, t12132: F, t588: F, t39253: F, t9453: F, t2411: F, t2414: F, t701: F, t9777: F, t2405: F, t2415: F) -> (F, F, F, F, F, F, F) {
        let t39563 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1126::<F>(t39273, t39275, t39278, t39281, t39284, t39289, t39291, t39293, t39295, t39298, t683, t702);
        let (t39568, t39570, t39582, t39585) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1127::<F>(t39378, t746, t9720, t1294, t12132, t588, t39253, t702, t9453);
        let t39590 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1128::<F>(t2411, t2414, t701, t9777);
        let t39593 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1129::<F>(t2405, t2415, t9453);
    (t39563, t39568, t39570, t39582, t39585, t39590, t39593)
}
