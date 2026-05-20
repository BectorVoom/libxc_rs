//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta458<F: Float>(t4049: F, t5396: F, t20215: F, t95: F, t5415: F, t1449: F, t5480: F, t9398: F, t4059: F, t5484: F, t103: F, t100: F, t104: F, t1447: F, t1450: F, t20312: F, t5475: F, t5481: F, t5485: F, t92: F, tau1: F, t109: F, t656: F, t12747: F, t19471: F, t19480: F, t20305: F, t20308: F, t64: F, t9358: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20315, t20318, t20319, t20322, t20331, t20338, t20342) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1829::<F>(t4049, t5396, t20215, t95, t5415, t1449, t5480, t9398, t4059, t5484, t103, t100, t104, t1447, t1450, t20312, t5475, t5481, t5485, t92, tau1);
        let (t20343, t20347) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1830::<F>(t109, t20342, t656, t12747, t19471, t19480, t20305, t20308, t64, t9358);
    (t20315, t20318, t20319, t20322, t20331, t20338, t20342, t20343, t20347)
}
