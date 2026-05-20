//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1251;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1252;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1253;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1254;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1255;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1256;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1257;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta347<F: Float>(t41654: F, t2394: F, t2781: F, t2772: F, t10565: F, t690: F, t10574: F, t10969: F, t154: F, t2769: F, t39097: F, t123: F, t39110: F, t883: F, t882: F, t2777: F, t41642: F, t41646: F, t41651: F, t10568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41655, t41656) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1251::<F>(t41654, t2394, t2781);
        let t41658 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1252::<F>(t2394, t2772);
        let t41660 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1253::<F>(t10565, t690);
        let t41662 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1254::<F>(t10574, t690);
        let (t41666, t41667, t41669) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1255::<F>(t10969, t154, t2769, t39097, t123);
        let (t41671, t41673) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1256::<F>(t39110, t883, t123, t882);
        let t41675 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1257::<F>(t2394, t2777);
        let (t41677, t41678) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1258::<F>(t41642, t41646, t41651, t41655, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t10568, t690);
    (t41656, t41658, t41660, t41662, t41666, t41667, t41669, t41671, t41673, t41675, t41677, t41678)
}
