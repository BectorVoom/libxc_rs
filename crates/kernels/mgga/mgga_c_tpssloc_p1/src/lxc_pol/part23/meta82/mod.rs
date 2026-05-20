//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk481;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk482;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk483;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta82<F: Float>(t786: F, t792: F, t59: F, t835: F, t154: F, t116: F, t206: F, t212: F, t2559: F, t222: F, t233: F, t813: F, t236: F, t240: F, t812: F, t232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2576, t2585, t2586) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk481::<F>(t786, t792, t59, t835, t154);
        let (t2588, t2590, t2600, t2602, t2627) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk482::<F>(t116, t206, t212, t2586, t154, t2559, t222, t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk483::<F>(t236, t2627);
        let (t2629, t2630, t2632) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk484::<F>(t240, t2628, t812, t232);
    (t2576, t2585, t2586, t2588, t2590, t2600, t2602, t2627, t2628, t2629, t2630, t2632)
}
