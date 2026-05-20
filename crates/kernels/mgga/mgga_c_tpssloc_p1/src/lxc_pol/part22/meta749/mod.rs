//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta749 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2503;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2504;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2505;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2506;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2507;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2508;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2509;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2510;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta749<F: Float>(t47774: F, t50992: F, t68513: F, t20234: F, t43791: F, t607: F, t11145: F, t123: F, t20217: F, t3242: F, t3240: F, t21766: F, t690: F, t21773: F, t21759: F, t1089: F, t67060: F, t1088: F, t21770: F, t21777: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t71130 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2503::<F>(t47774, t50992, t68513);
        let (t71133, t71135) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2504::<F>(t20234, t43791, t607, t11145, t123);
        let (t71138, t71140) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2505::<F>(t20217, t3242, t607, t123, t3240);
        let t71142 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2506::<F>(t21766, t690);
        let t71144 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2507::<F>(t21773, t690);
        let t71146 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2508::<F>(t21759, t690);
        let (t71148, t71150) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2509::<F>(t1089, t67060, t1088, t123);
        let t71152 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2510::<F>(t21770, t690);
        let t71154 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2511::<F>(t21777, t690);
    (t71130, t71133, t71135, t71138, t71140, t71142, t71144, t71146, t71148, t71150, t71152, t71154)
}
