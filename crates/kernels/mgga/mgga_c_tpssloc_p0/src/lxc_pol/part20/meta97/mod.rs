//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta97 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk661;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk662;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk663;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk664;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta97<F: Float>(t138: F, t681: F, t125: F, t2412: F, t702: F, t118: F, t142: F, t2393: F, t706: F, t717: F, t708: F, t607: F, t751: F, t707: F, t195: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2418, t2419, t2420, t2421, t2423) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk661::<F>(t138, t681, t125, t2412, t702);
        let t2426 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk662::<F>(t118, t142, t2393);
        let t2427 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk663::<F>(t706, t717);
        let (t2429, t2430) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk664::<F>(t2427, t708, t607, t751);
        let (t2431, t2432, t2433) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk665::<F>(t2430, t707, t195);
    (t2418, t2419, t2420, t2421, t2423, t2426, t2427, t2429, t2430, t2431, t2432, t2433)
}
