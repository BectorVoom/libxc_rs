//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta97 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk542;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk543;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk544;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk545;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk546;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk547;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta97<F: Float>(t3400: F, t440: F, t448: F, t457: F, t697: F, t461: F, t221: F, t456: F, t1176: F, t135: F, t1089: F, t405: F, t974: F, t3242: F, t337: F, t51: F, t1887: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3401 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk542::<F>(t3400, t440);
        let (t3402, t3403) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk543::<F>(t448);
        let (t3426, t3428, t3430, t3431) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk544::<F>(t457, t697, t461, t221, t456, t1176, t135);
        let t3439 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk545::<F>(t1089, t405);
        let t3440 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk546::<F>(t3439, t974);
        let (t3441, t3447) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk547::<F>(t3242, t461, t337, t51, t1887);
        let t3448 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk548::<F>(t1176, t60);
    (t3401, t3402, t3403, t3426, t3428, t3430, t3431, t3439, t3440, t3441, t3447, t3448)
}
