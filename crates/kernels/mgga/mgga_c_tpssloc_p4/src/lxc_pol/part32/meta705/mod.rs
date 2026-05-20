//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2204;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta705<F: Float>(t26161: F, t26163: F, t97920: F, t24991: F, t7685: F, t22574: F, t25988: F, t33136: F, t28823: F, t6876: F, t1874: F, t96709: F, t19534: F, t89: F, t28030: F, t6525: F, t28821: F, t6880: F, t28239: F, t1983: F, t26503: F, t5161: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97923, t97925, t97928, t97930, t97932) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2204::<F>(t26161, t26163, t97920, t24991, t7685, t22574, t25988, t33136, t28823, t6876, t1874, t96709);
        let (t97935, t97937, t97941, t97942, t97947) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2205::<F>(t19534, t89, t1874, t28030, t6525, t28821, t6880, t28239, t6876, t1983, t26503, t5161);
    (t97923, t97925, t97928, t97930, t97932, t97935, t97937, t97941, t97942, t97947)
}
