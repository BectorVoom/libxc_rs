//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1879;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta623<F: Float>(t1307: F, t6637: F, t6888: F, t97126: F, t26331: F, t26446: F, t96964: F, t28164: F, t6914: F, t22704: F, t22705: F, t28181: F, t19889: F, t91004: F, t91006: F, t28182: F, t19660: F, t22633: F, t3807: F, t6976: F, t22685: F, t22881: F, t6330: F) -> (F, F, F, F, F, F, F, F) {
        let (t97129, t97135, t97137, t97142) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1879::<F>(t1307, t6637, t6888, t97126, t26331, t26446, t96964, t28164, t6914, t22704, t22705, t28181);
        let (t97146, t97148, t97152, t97158) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1880::<F>(t19889, t91004, t91006, t28182, t6914, t19660, t22633, t3807, t6976, t22685, t22881, t6330, t6637);
    (t97129, t97135, t97137, t97142, t97146, t97148, t97152, t97158)
}
