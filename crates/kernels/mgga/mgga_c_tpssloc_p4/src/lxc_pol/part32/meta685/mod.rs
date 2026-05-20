//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta685<F: Float>(t19334: F, t605: F, t2235: F, t5392: F, t19534: F, t88: F, t1873: F, t28007: F, t6534: F, t26114: F, t7467: F, t26117: F) -> (F, F, F, F, F, F) {
        let (t96562, t96646, t96659, t96661, t96663, t96665) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2127::<F>(t19334, t605, t2235, t5392, t19534, t88, t1873, t28007, t6534, t26114, t7467, t26117);
    (t96562, t96646, t96659, t96661, t96663, t96665)
}
