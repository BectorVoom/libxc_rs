//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2483;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta677<F: Float>(t4199: F, t9494: F, t13471: F, t870: F, t12945: F, t2427: F, t12858: F, t2528: F, t2371: F, t4205: F, t9909: F, t13123: F, t9885: F, t12908: F, t12924: F, t4101: F, t9912: F, t1409: F, t2516: F, t4194: F, t607: F, t9722: F, t12887: F, t172: F, t763: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46208, t46213, t46217, t46234, t46236, t46244, t46278) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2483::<F>(t4199, t9494, t13471, t870, t12945, t2427, t12858, t2528, t2371, t4205, t9909, t13123, t9885);
        let (t46283, t46285, t46291, t46302, t46308) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2484::<F>(t12908, t12924, t4101, t9912, t1409, t2516, t4194, t607, t4199, t9722, t12887, t172, t763);
    (t46208, t46213, t46217, t46234, t46236, t46244, t46278, t46283, t46285, t46291, t46302, t46308)
}
