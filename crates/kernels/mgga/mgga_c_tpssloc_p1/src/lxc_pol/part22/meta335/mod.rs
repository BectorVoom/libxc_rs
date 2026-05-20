//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1528;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1529;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta335<F: Float>(t111: F, t1851: F, t5392: F, t9427: F, t2433: F, t5398: F, t12603: F, t12604: F, t25: F, t28: F, zeta_threshold: F, t40: F, t52: F, t3966: F, t4080: F, t607: F, t73: F, t9438: F, t2440: F, t4087: F, t76: F) -> (F, F, F, F, F, F, F) {
        let (t16524, t16549, t16554, t16557) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1528::<F>(t111, t1851, t5392, t9427, t2433, t5398, t12603, t12604);
        let t16558 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1529::<F>(t25, t28, t16557, zeta_threshold);
        let (t16562, t16563, t16574) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1530::<F>(t40, t52, t16549, t16554, t16558, t3966, t4080, t607, t73, t5392, t9438, t2440, t5398, t4087, t76, zeta_threshold);
    (t16524, t16549, t16557, t16558, t16562, t16563, t16574)
}
