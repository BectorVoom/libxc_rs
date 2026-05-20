//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1866;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1867;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1868;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1869;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta436<F: Float>(t14722: F, t14704: F, t11147: F, t1409: F, t2244: F, t11145: F, t123: F, t11153: F, t3240: F, t3242: F, t3966: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14723, t14724, t14725, t14726) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1866::<F>(t14722, t14704, t11147, t1409, t2244);
        let (t14727, t14728) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1867::<F>(t11145, t14726, t123);
        let (t14730, t14731) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1868::<F>(t11153, t1409, t2244);
        let (t14732, t14733) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1869::<F>(t14731, t3240, t123);
        let (t14735, t14736) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1870::<F>(t3242, t3966, t607);
    (t14723, t14724, t14725, t14726, t14727, t14728, t14730, t14731, t14732, t14733, t14735, t14736)
}
