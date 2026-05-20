//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta474<F: Float>(t16535: F, t1873: F, t6534: F, t671: F, t3941: F, t2363: F, t1401: F, t22479: F, t2274: F, t50: F, t2244: F, t2250: F, t22510: F, t7251: F) -> (F, F, F, F, F, F, F, F) {
        let (t23892, t23893, t23895, t23896, t23898, t23900, t24498, t24503) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1810::<F>(t16535, t1873, t6534, t671, t3941, t2363, t1401, t22479, t2274, t50, t2244, t2250, t22510, t7251);
    (t23892, t23893, t23895, t23896, t23898, t23900, t24498, t24503)
}
