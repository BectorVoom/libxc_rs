//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta529<F: Float>(t1240: F, t1760: F, t2122: F, t1186: F, t4733: F, t7286: F, t7285: F, t1716: F, t24638: F, t7391: F, t3598: F, t24574: F, t8003: F, t7295: F, t6686: F, t8020: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27381, t27382, t27383, t27388, t27389, t27392, t27396, t27401) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1865::<F>(t1240, t1760, t2122, t1186, t4733, t7286, t7285, t1716, t24638, t7391, t3598, t24574, t8003);
        let (t27403, t27406) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1866::<F>(t1716, t7295, t6686, t8020);
    (t27381, t27382, t27383, t27388, t27389, t27392, t27396, t27401, t27403, t27406)
}
