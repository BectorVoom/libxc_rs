//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2148/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2148<F: Float>(t28164: F, t6914: F, t22704: F, t22705: F, t28181: F, t19889: F, t91004: F, t91006: F, t28182: F, t19660: F, t22633: F, t3807: F, t6976: F) -> (F, F, F, F, F) {
    let t97137 = t6914 * t28164;
    let t97142 = t22704 * t22705 * t28181;
    let t97146 = t91004 * t91006 * t19889;
    let t97148 = t6914 * t28182;
    let t97152 = t22633 * t6976 * t19660 * t3807;
    (t97137, t97142, t97146, t97148, t97152)
}
