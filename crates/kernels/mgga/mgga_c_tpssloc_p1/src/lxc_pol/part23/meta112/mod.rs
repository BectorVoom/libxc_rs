//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk589;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk590;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta112<F: Float>(t226: F, t4280: F, t1509: F, t252: F, t68: F, t814: F, t1519: F, t1530: F, t870: F, t193: F, t200: F, t1484: F, t262: F, t1540: F, t690: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4281, t4282, t4290, t4291, t4295, t4310) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk589::<F>(t226, t4280, t1509, t252, t68, t814, t1519, t1530, t870);
        let t4314 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk590::<F>(t193, t200);
        let (t4315, t4335) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk591::<F>(t1484, t262, t1540, t690);
    (t4281, t4282, t4290, t4291, t4295, t4310, t4314, t4315, t4335)
}
