//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2320;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2321;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2322;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta586<F: Float>(t3726: F, t6358: F, t213: F, t6347: F, t1307: F, t221: F, t12228: F, t12236: F, t16078: F, t16083: F, t16099: F, t16106: F, t16108: F, t16113: F, t16119: F, t5195: F, t19790: F, t225: F, t1814: F, t5343: F, t3901: F, t6420: F, t6378: F, t68: F) -> (F, F, F, F, F, F, F) {
        let (t19793, t19795, t19803) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2320::<F>(t3726, t6358, t213, t6347, t1307, t221, t12228, t12236, t16078, t16083, t16099, t16106, t16108, t16113, t16119, t5195);
        let (t19804, t19805, t19810) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2321::<F>(t19790, t19803, t225, t1814, t5343);
        let (t19813, t19815) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2322::<F>(t3901, t6420, t6378, t68);
    (t19793, t19795, t19804, t19805, t19810, t19813, t19815)
}
