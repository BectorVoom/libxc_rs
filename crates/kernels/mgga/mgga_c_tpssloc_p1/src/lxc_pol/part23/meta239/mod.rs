//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta239<F: Float>(t1020: F, t17611: F, t135: F, t5889: F, t973: F, t5893: F, t5884: F, t248: F, t3101: F, t5878: F, t3039: F, t3051: F, t5685: F) -> (F, F, F, F, F, F, F) {
        let (t17612, t17616, t17621, t17625, t17655, t17656, t17659) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk894::<F>(t1020, t17611, t135, t5889, t973, t5893, t5884, t248, t3101, t5878, t3039, t3051, t5685);
    (t17612, t17616, t17621, t17625, t17655, t17656, t17659)
}
