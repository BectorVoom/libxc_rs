//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1880/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1880<F: Float>(t19889: F, t91004: F, t91006: F, t28182: F, t6914: F, t19660: F, t22633: F, t3807: F, t6976: F, t22685: F, t22881: F, t6330: F, t6637: F) -> (F, F, F, F) {
    let t97146 = t91004 * t91006 * t19889;
    let t97148 = t6914 * t28182;
    let t97152 = t22633 * t6976 * t19660 * t3807;
    let t97158 = t22685 * t6637 * t22881 * t6330;
    (t97146, t97148, t97152, t97158)
}
