//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta655<F: Float>(t1873: F, t55934: F, t12725: F, t6534: F, t55962: F, t19456: F, t4072: F, t649: F, t26114: F, t12813: F, t88: F, t22479: F, t4028: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90363, t90365, t90367, t90369, t90370, t90372, t90374, t90377, t90379) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2181::<F>(t1873, t55934, t12725, t6534, t55962, t19456, t4072, t649, t26114, t12813, t88, t22479, t4028);
    (t90363, t90365, t90367, t90369, t90370, t90372, t90374, t90377, t90379)
}
