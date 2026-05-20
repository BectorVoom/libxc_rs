//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1915;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta607<F: Float>(t22892: F, t22893: F, t26384: F, t16018: F, t6637: F, t6888: F, t6968: F, t26388: F, t7733: F, t81186: F, t5318: F, t552: F, t1307: F, t1352: F, t22633: F, t6976: F, t90754: F, t5187: F, t562: F, t1799: F, t81129: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90797, t90801, t90805, t90807, t90809) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1915::<F>(t22892, t22893, t26384, t16018, t6637, t6888, t6968, t26388, t7733, t81186, t5318, t552);
        let (t90812, t90816, t90818, t90821, t90825) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1916::<F>(t1307, t6637, t6888, t90809, t1352, t22633, t6976, t90754, t5187, t562, t1799, t81129);
    (t90797, t90801, t90805, t90807, t90812, t90816, t90818, t90821, t90825)
}
