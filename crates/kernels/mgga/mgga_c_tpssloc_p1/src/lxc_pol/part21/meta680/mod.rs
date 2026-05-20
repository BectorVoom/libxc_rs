//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2489;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta680<F: Float>(t4119: F, t828: F, t46528: F, t842: F, t4261: F, t9601: F, t1516: F, t40965: F, t13347: F, t2697: F, t13210: F, t9638: F, t120: F, t13170: F, t13231: F, t13258: F, t41107: F, t4250: F, t13244: F, t242: F, t812: F, t841: F, t1484: F, t2678: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46565, t46570, t46573, t46577, t46587, t46595) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2489::<F>(t4119, t828, t46528, t842, t4261, t9601, t1516, t40965, t13347, t2697, t13210, t9638);
        let (t46597, t46611, t46616, t46618, t46628, t46644) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2490::<F>(t120, t13170, t13231, t13258, t41107, t4250, t13244, t242, t812, t841, t1484, t2678);
    (t46565, t46570, t46573, t46577, t46587, t46595, t46597, t46611, t46616, t46618, t46628, t46644)
}
