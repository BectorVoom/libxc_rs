//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 993/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk993<F: Float>(t32852: F, t858: F, t30640: F, t32791: F, t32794: F, t32796: F, t32800: F, t32804: F, t32811: F, t32817: F, t4147: F, t4268: F, t6627: F, t7517: F, t8353: F, t8363: F, t855: F) -> (F, F) {
    let t32853 = t858 * t32852;
    let t32860 = -6.0 * t32796 * t855 + 4.0 * t32800 * t855 + 2.0 * t32804 * t855 - t32853 * t855 + 2.0 * t4147 * t8353 + 2.0 * t4268 * t8353 - t4268 * t8363 + 4.0 * t6627 * t7517 - t30640 - t32791 - t32794 + t32811 + t32817;
    (t32853, t32860)
}
