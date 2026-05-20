//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta566<F: Float>(t1527: F, t7537: F, t2718: F, t1911: F, t5636: F, t10110: F, t5657: F, t16815: F, t232: F, t6646: F, t1888: F, t5544: F, t6638: F) -> (F, F, F, F, F, F, F) {
        let (t28307, t28311, t28317, t28321, t28322, t28323, t28329) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1933::<F>(t1527, t7537, t2718, t1911, t5636, t10110, t5657, t16815, t232, t6646, t1888, t5544, t6638);
    (t28307, t28311, t28317, t28321, t28322, t28323, t28329)
}
