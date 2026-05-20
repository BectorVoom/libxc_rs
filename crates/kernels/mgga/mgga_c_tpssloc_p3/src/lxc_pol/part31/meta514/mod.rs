//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta514<F: Float>(t17030: F, t232: F, t6646: F, t1888: F, t16815: F, t2632: F, t22996: F, t25224: F, t7488: F, t1880: F, t25: F, t5664: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28422, t28423, t28424, t28426, t28427, t28428, t28439, t28440, t28456) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1710::<F>(t17030, t232, t6646, t1888, t16815, t2632, t22996, t25224, t7488, t1880, t25, t5664);
    (t28422, t28423, t28424, t28426, t28427, t28428, t28439, t28440, t28456)
}
