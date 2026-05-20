//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk765;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk766;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk767;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk768;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk769;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk770;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk771;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta113<F: Float>(t1229: F, t154: F, t636: F, t2296: F, t1094: F, t1098: F, t1097: F, t419: F, t409: F, t407: F, t410: F, t3236: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3240 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk765::<F>(t1229, t154);
        let t3241 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk766::<F>(t636);
        let t3242 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk767::<F>(t3241);
        let t3247 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk768::<F>(t2296);
        let (t3259, t3262, t3263) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk769::<F>(t1094, t1098, t1097, t419);
        let t3264 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk770::<F>(t3263, t409);
        let t3270 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk771::<F>(t407, t410);
        let (t3274, t3282, t3287) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk772::<F>(t3236, t407);
    (t3240, t3241, t3242, t3247, t3259, t3262, t3263, t3264, t3270, t3274, t3282, t3287)
}
