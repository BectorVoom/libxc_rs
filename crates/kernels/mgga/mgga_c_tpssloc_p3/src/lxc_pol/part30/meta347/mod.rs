//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta347<F: Float>(t13913: F, t973: F, t13552: F, t13550: F, t13644: F, t1036: F, t4622: F, t3117: F, t4571: F, t248: F, t3051: F, t4347: F) -> (F, F, F, F, F, F, F) {
        let (t13915, t13921, t13922, t13923, t13946, t13948, t13950) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1387::<F>(t13913, t973, t13552, t13550, t13644, t1036, t4622, t3117, t4571, t248, t3051, t4347);
    (t13915, t13921, t13922, t13923, t13946, t13948, t13950)
}
