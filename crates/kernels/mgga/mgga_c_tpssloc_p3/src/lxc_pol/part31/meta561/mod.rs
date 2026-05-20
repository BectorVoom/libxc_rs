//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta561<F: Float>(t82147: F, t1887: F, t81956: F, t25041: F, t215: F, t6581: F, t252: F, t81613: F, t23056: F, t25242: F, t6579: F, t25245: F, t82031: F) -> (F, F, F, F, F, F, F, F) {
        let (t87042, t87049, t87050, t87052, t87053, t87057, t87066, t87068) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1790::<F>(t82147, t1887, t81956, t25041, t215, t6581, t252, t81613, t23056, t25242, t6579, t25245, t82031);
    (t87042, t87049, t87050, t87052, t87053, t87057, t87066, t87068)
}
