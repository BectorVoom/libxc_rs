//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk810;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta182<F: Float>(t261: F, t2751: F, t1053: F, t68: F, t134: F, t976: F, t271: F, t2775: F, t974: F, t2769: F, t632: F) -> (F, F, F, F, F, F, F) {
        let (t10143, t10163, t10165, t10189, t10213) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk810::<F>(t261, t2751, t1053, t68, t134, t976, t271, t2775);
        let (t10214, t10216) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk811::<F>(t10213, t974, t2769, t632);
    (t10143, t10163, t10165, t10189, t10213, t10214, t10216)
}
