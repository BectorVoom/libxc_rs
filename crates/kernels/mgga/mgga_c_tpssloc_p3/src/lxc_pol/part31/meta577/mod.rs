//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta577<F: Float>(t22751: F, t26186: F, t26190: F, t26356: F, t6914: F, t1799: F, t3886: F, t80663: F, t80671: F, t1887: F, t80827: F, t26334: F) -> (F, F, F, F, F, F, F, F) {
        let (t90468, t90470, t90472, t90488, t90493, t90496, t90497, t90498) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1813::<F>(t22751, t26186, t26190, t26356, t6914, t1799, t3886, t80663, t80671, t1887, t80827, t26334);
    (t90468, t90470, t90472, t90488, t90493, t90496, t90497, t90498)
}
