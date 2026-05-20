//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta375<F: Float>(t4199: F, t9494: F, t13123: F, t9885: F, t9722: F, t1409: F, t707: F, t9862: F, t9467: F, t9713: F, t1471: F, t31: F) -> (F, F, F, F, F, F, F) {
        let (t46208, t46278, t46302, t46369, t46371, t46376, t46387) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1176::<F>(t4199, t9494, t13123, t9885, t9722, t1409, t707, t9862, t9467, t9713, t1471, t31);
    (t46208, t46278, t46302, t46369, t46371, t46376, t46387)
}
