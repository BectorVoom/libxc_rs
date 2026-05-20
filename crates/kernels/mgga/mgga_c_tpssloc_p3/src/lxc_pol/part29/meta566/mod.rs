//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta566<F: Float>(t3700: F, t2751: F, t10108: F, t257: F, t3639: F, t11604: F, t496: F, t111: F, t3931: F, t12723: F, t1406: F, t9238: F) -> (F, F, F, F, F, F, F, F) {
        let (t40611, t40772, t40889, t43706, t45349, t45560, t45632, t45844) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1983::<F>(t3700, t2751, t10108, t257, t3639, t11604, t496, t111, t3931, t12723, t1406, t9238);
    (t40611, t40772, t40889, t43706, t45349, t45560, t45632, t45844)
}
