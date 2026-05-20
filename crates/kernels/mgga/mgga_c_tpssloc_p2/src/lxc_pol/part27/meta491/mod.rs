//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta491<F: Float>(t25: F, t870: F, t4255: F, t16596: F, t22960: F, t1484: F, t606: F, t4119: F, t7484: F, t794: F, t6562: F, t1887: F, t23056: F) -> (F, F, F, F, F, F, F, F) {
        let (t25014, t25015, t25021, t25024, t25028, t25035, t25036, t25038) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1877::<F>(t25, t870, t4255, t16596, t22960, t1484, t606, t4119, t7484, t794, t6562, t1887, t23056);
    (t25014, t25015, t25021, t25024, t25028, t25035, t25036, t25038)
}
