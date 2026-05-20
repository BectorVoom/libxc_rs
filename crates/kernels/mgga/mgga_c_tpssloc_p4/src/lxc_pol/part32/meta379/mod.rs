//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1436;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1437;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1438;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1439;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta379<F: Float>(t3726: F, t5227: F, t3802: F, t5234: F, t3788: F, t836: F, t1336: F, t5252: F, t225: F, t5319: F, t5217: F, t1390: F, t5356: F, t112: F, t5363: F, t111: F, t1851: F, t5392: F, t9427: F, t2433: F, t5398: F, t12603: F, t12604: F, t25: F, t28: F, zeta_threshold: F, t40: F, t52: F, t3966: F, t4080: F, t607: F, t73: F, t9438: F, t2440: F, t4087: F, t76: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16354, t16394, t16400, t16439, t16460, t16497) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1436::<F>(t3726, t5227, t3802, t5234, t3788, t836, t1336, t5252, t225, t5319, t5217, t1390, t5356);
        let (t16521, t16524) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1437::<F>(t112, t5363, t111, t1851);
        let (t16549, t16554, t16557) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1438::<F>(t5392, t9427, t2433, t5398, t12603, t12604);
        let t16558 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1439::<F>(t25, t28, t16557, zeta_threshold);
        let (t16562, t16574) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1440::<F>(t40, t52, t16549, t16554, t16558, t3966, t4080, t607, t73, t5392, t9438, t2440, t5398, t4087, t76, zeta_threshold);
    (t16354, t16394, t16400, t16439, t16460, t16497, t16521, t16524, t16557, t16558, t16562, t16574)
}
