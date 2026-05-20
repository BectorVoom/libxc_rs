//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1269;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta432<F: Float>(t19033: F, t4993: F, t19046: F, t5018: F, t5023: F, t6169: F, t18321: F, t5040: F, t1009: F, t22113: F, t1011: F, t1212: F, t18375: F, t5002: F, t1730: F, t19032: F, t1017: F, t1207: F, t1210: F, t22173: F, t372: F, t471: F, t479: F, t15507: F, t19095: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t72302, t72304, t72307, t72352, t72361, t72363) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1269::<F>(t19033, t4993, t19046, t5018, t5023, t6169, t18321, t5040, t1009, t22113, t1011, t1212);
        let (t72366, t72384, t72389, t72398, t72403) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1270::<F>(t18375, t5002, t1730, t19032, t1017, t1207, t1210, t22173, t372, t471, t479, t15507, t19095);
    (t72302, t72304, t72307, t72352, t72361, t72363, t72366, t72384, t72389, t72398, t72403)
}
