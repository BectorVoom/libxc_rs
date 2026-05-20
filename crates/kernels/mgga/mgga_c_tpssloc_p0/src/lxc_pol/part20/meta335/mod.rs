//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1625;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1626;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta335<F: Float>(t11691: F, t11757: F, t11817: F, t11866: F, t493: F, t3493: F, t3612: F, t1245: F, t11812: F, t1243: F, t10471: F, t11715: F, t11712: F, t11720: F, t491: F, t11721: F, t6739: F, t3502: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11868, t11869, t11871, t11872, t11877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1625::<F>(t11691, t11757, t11817, t11866, t493, t3493, t3612, t1245, t11812, t1243);
        let (t11880, t11881) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1626::<F>(t10471, t11715, t11712);
        let (t11882, t11883, t11884, t11887, t11888) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1627::<F>(t11720, t491, t11721, t6739, t10471, t3502, t11712);
    (t11868, t11869, t11871, t11872, t11877, t11880, t11881, t11882, t11883, t11884, t11887, t11888)
}
