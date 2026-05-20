//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1402;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1403;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1404;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1405;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1406;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta471<F: Float>(t11147: F, t75836: F, t136: F, t3297: F, t11153: F, t1113: F, t1089: F, t75912: F, t1088: F, t123: F, t43809: F, t77961: F, t77969: F, t50834: F, t71335: F, t71337: F, t77959: F, t77963: F, t77967: F, t77971: F, t11145: F, t77957: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t77973, t77975, t77977, t77979, t77981, t77983, t77989) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1402::<F>(t11147, t75836, t136, t3297, t11153, t1113, t1089, t75912, t1088, t123);
        let t77992 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1403::<F>(t1088, t123, t77981);
        let t77995 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1404::<F>(t123, t43809, t77961);
        let t77998 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1405::<F>(t1088, t123, t77969);
        let t78000 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1406::<F>(t50834, t71335, t71337, t77959, t77963, t77967, t77971, t77975, t77979, t77983, t77989, t77992, t77995, t77998);
        let t78002 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1407::<F>(t11145, t123, t77957);
    (t77973, t77975, t77977, t77979, t77981, t77983, t77989, t77992, t77995, t77998, t78000, t78002)
}
