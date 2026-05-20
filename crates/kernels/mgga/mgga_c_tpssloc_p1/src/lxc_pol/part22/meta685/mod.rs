//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2256;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2257;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2258;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2259;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2260;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2261;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta685<F: Float>(t18242: F, t690: F, t2394: F, t5976: F, t18216: F, t18212: F, t18226: F, t18222: F, t3375: F, t6063: F, t18893: F, t3359: F, t11285: F, t6084: F, t18785: F, t3403: F, t18834: F, t3315: F, t1147: F, t18710: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t63336 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2256::<F>(t18242, t690);
        let t63361 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2257::<F>(t2394, t5976);
        let t63382 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2258::<F>(t18216, t690);
        let t63384 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2259::<F>(t18212, t690);
        let t63398 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2260::<F>(t18226, t690);
        let t63400 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2261::<F>(t18222, t690);
        let (t63454, t63502, t63519, t63533, t63588, t63597) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2262::<F>(t3375, t6063, t18893, t3359, t11285, t6084, t18785, t3403, t18834, t3315, t1147, t18710);
    (t63336, t63361, t63382, t63384, t63398, t63400, t63454, t63502, t63519, t63533, t63588, t63597)
}
