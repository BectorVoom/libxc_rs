//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1177;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta376<F: Float>(t4211: F, t9874: F, t1472: F, t9862: F, t1519: F, t9971: F, t1496: F, t41083: F, t1516: F, t40965: F, t4166: F, t9637: F, t12985: F, t9577: F, t41189: F, t4134: F, t1489: F, t133: F, t1484: F, t41214: F, t6600: F, t1512: F, t41362: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46433, t46439, t46524, t46546, t46577, t46657) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1177::<F>(t4211, t9874, t1472, t9862, t1519, t9971, t1496, t41083, t1516, t40965, t4166, t9637);
        let (t46764, t46772, t46790, t46806, t46876) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1178::<F>(t12985, t9577, t41189, t4134, t1489, t41083, t133, t1484, t41214, t6600, t1512, t41362);
    (t46433, t46439, t46524, t46546, t46577, t46657, t46764, t46772, t46790, t46806, t46876)
}
