//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta687<F: Float>(t1512: F, t41354: F, t13198: F, t2697: F, t13302: F, t9638: F, t13306: F, t13248: F, t13258: F, t1484: F, t2631: F, t4233: F, t828: F) -> (F, F, F, F, F, F, F) {
        let (t46960, t46962, t46974, t46980, t46998, t47012, t47017) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2502::<F>(t1512, t41354, t13198, t2697, t13302, t9638, t13306, t13248, t13258, t1484, t2631, t4233, t828);
    (t46960, t46962, t46974, t46980, t46998, t47012, t47017)
}
