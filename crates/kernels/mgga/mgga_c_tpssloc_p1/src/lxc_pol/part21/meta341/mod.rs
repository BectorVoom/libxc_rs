//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1733;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta341<F: Float>(t40: F, t1409: F, t9427: F, t2433: F, t3966: F, t12606: F, t2244: F, t2250: F, t4080: F, t607: F, t73: F, t9438: F, t2440: F, zeta_threshold: F, t52: F, t4087: F, t76: F, t157: F, t182: F, t145: F) -> (F, F, F, F, F, F) {
        let (t12862, t12873, t12874, t12877) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1733::<F>(t40, t1409, t9427, t2433, t3966, t12606, t2244, t2250, t4080, t607, t73, t9438, t2440, zeta_threshold);
        let (t12886, t12887, t12889, t12890) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1734::<F>(t52, t12606, t12874, t12877, t2244, t2250, t4087, t607, t76, t12873, t157, t182, t145, zeta_threshold);
    (t12862, t12874, t12886, t12887, t12889, t12890)
}
