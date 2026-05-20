//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk655;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk656;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk657;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk658;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk659;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta96<F: Float>(t2405: F, t702: F, t683: F, t681: F, t125: F, t701: F, t141: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2406, t2408) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk655::<F>(t2405, t702, t683);
        let t2409 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk656::<F>(t681);
        let (t2410, t2411) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk657::<F>(t2409, t125);
        let t2412 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk658::<F>(t701);
        let (t2413, t2414) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk659::<F>(t141);
        let (t2415, t2417) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk660::<F>(t2412, t2414, t2411);
    (t2406, t2408, t2409, t2410, t2411, t2412, t2413, t2414, t2415, t2417)
}
