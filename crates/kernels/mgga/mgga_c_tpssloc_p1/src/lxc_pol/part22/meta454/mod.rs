//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1819;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta454<F: Float>(t20118: F, t20147: F, t3: F, t112: F, t6470: F, t576: F, t671: F, t1458: F, t4072: F, t5493: F, t12524: F, t1401: F, t16521: F, t16524: F, t19534: F, t3938: F, t3941: F, t5371: F, t5376: F, t5456: F, t577: F, t9211: F, t9213: F, t9215: F, t9217: F, t9219: F, t9221: F, t9225: F, t1437: F, t5389: F, t5445: F, t1864: F, t5398: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20148, t20149, t20162, t20173, t20176, t20181, t20186) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1819::<F>(t20118, t20147, t3, t112, t6470, t576, t671, t1458, t4072, t5493, t12524, t1401, t16521, t16524, t19534, t3938, t3941, t5371, t5376, t5456, t577);
        let (t20193, t20201, t20204, t20207) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1820::<F>(t9211, t9213, t9215, t9217, t9219, t9221, t9225, t1437, t5389, t5445, t1864, t5398);
    (t20148, t20149, t20162, t20173, t20176, t20181, t20186, t20193, t20201, t20204, t20207)
}
