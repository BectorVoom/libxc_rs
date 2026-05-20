//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta309<F: Float>(t2929: F, t938: F, t10523: F, t315: F, t10544: F, t1004: F, t3047: F, t3053: F, t3117: F, t1043: F, t676: F, t248: F, t884: F) -> (F, F, F, F, F, F, F) {
        let (t10825, t10828, t10832, t10863, t10866, t10868, t10870) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1376::<F>(t2929, t938, t10523, t315, t10544, t1004, t3047, t3053, t3117, t1043, t676, t248, t884);
    (t10825, t10828, t10832, t10863, t10866, t10868, t10870)
}
