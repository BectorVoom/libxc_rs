//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta426<F: Float>(t252: F, t2631: F, t2632: F, t22996: F, t1888: F, t6579: F, t6649: F, t232: F, t6646: F, t1879: F, t22715: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22998, t22999, t23000, t23002, t23003, t23004, t23005, t23006, t23012) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1603::<F>(t252, t2631, t2632, t22996, t1888, t6579, t6649, t232, t6646, t1879, t22715);
    (t22998, t22999, t23000, t23002, t23003, t23004, t23005, t23006, t23012)
}
