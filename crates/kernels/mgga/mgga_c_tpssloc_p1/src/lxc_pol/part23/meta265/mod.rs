//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk934;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta265<F: Float>(t112: F, t20292: F, t1441: F, t5456: F, t1453: F, t5464: F, t9365: F, t4043: F, t5488: F, t1444: F, t5468: F, t9384: F, t4049: F, t5396: F, t20215: F, t95: F, t5415: F, t1449: F, t5480: F, t9398: F, t4059: F, t5484: F, t103: F, t100: F, t104: F, t1447: F, t1450: F, t5475: F, t5481: F, t5485: F, t92: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20293, t20296, t20305, t20308, t20312) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk934::<F>(t112, t20292, t1441, t5456, t1453, t5464, t9365, t4043, t5488, t1444, t5468, t9384);
        let (t20318, t20322, t20332, t20335, t20338, t20339, t20342) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk935::<F>(t4049, t5396, t20215, t95, t5415, t1449, t5480, t9398, t4059, t5484, t103, t100, t104, t1447, t1450, t20312, t5475, t5481, t5485, t92, tau1);
    (t20293, t20296, t20305, t20308, t20318, t20322, t20332, t20335, t20338, t20339, t20342)
}
