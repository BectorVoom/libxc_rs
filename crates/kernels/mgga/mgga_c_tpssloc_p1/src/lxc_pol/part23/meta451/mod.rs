//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1299;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1300;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta451<F: Float>(t5664: F, t67159: F, t58021: F, t46278: F, t67177: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t39483: F, t40741: F, t40743: F, t40748: F, t40760: F, t40764: F, t40766: F, t40772: F, t4314: F, t67154: F, t67235: F, t67179: F, t67185: F, t46302: F, t67209: F, t16: F, t39031: F, t25: F, t28: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t75884, t75885, t75886, t75887, t75891) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1299::<F>(t5664, t67159, t58021, t46278, t67177, t1484, t1530, t1877, t193, t202, t39483, t40741, t40743, t40748, t40760, t40764, t40766, t40772, t4314, t67154, t67235);
        let (t75894, t75895, t75900, t75901, t75910, t75911) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1300::<F>(t67179, t67185, t46302, t67209, t16, t39031);
        let t75912 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1301::<F>(t25, t28, t75911, zeta_threshold);
    (t75884, t75885, t75886, t75887, t75891, t75894, t75895, t75900, t75901, t75910, t75911, t75912)
}
