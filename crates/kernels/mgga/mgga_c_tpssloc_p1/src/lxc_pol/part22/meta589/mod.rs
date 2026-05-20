//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2102;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta589<F: Float>(t46573: F, t1516: F, t40965: F, t242: F, t812: F, t841: F, t41115: F, t4250: F, t4166: F, t9637: F, t13176: F, t2638: F, t4179: F, t820: F, t836: F, t9972: F, t12985: F, t9577: F, t212: F, t4119: F, t2586: F, t9523: F, t4138: F, t9541: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46574, t46577, t46628, t46650, t46657, t46667) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2102::<F>(t46573, t1516, t40965, t242, t812, t841, t41115, t4250, t4166, t9637, t13176, t2638);
        let (t46692, t46741, t46764, t46769, t46770) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2103::<F>(t4179, t820, t812, t836, t9972, t12985, t9577, t212, t4119, t2586, t9523, t4138, t9541);
    (t46574, t46577, t46628, t46650, t46657, t46667, t46692, t46741, t46764, t46769, t46770)
}
