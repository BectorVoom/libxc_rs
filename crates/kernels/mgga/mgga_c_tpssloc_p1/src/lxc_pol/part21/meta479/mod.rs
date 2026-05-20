//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2074;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta479<F: Float>(t40: F, t52: F, t16549: F, t16554: F, t16558: F, t3966: F, t4080: F, t607: F, t73: F, t5392: F, t9438: F, t2440: F, t5398: F, t4087: F, t76: F, zeta_threshold: F, t145: F, t185: F, t5520: F, t751: F, t157: F, t182: F, t12861: F, t4119: F, t4315: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16562, t16563, t16568, t16574) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2074::<F>(t40, t52, t16549, t16554, t16558, t3966, t4080, t607, t73, t5392, t9438, t2440, t5398, t4087, t76, zeta_threshold);
        let (t16575, t16576, t16577, t16578, t16579, t16581, t16582, t16583, t16586) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2075::<F>(t16562, t16574, t145, t185, t5520, t751, t157, t182, t12861, t4119, t4315, t5392);
    (t16563, t16568, t16575, t16576, t16577, t16578, t16579, t16581, t16582, t16583, t16586)
}
