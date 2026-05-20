//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk667;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk668;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk669;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk670;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk671;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta124<F: Float>(t3426: F, t461: F, t221: F, t456: F, t1176: F, t135: F, t1179: F, t1174: F, t1186: F, t1089: F, t405: F, t974: F, t3242: F, t2244: F, t337: F, t51: F, t1887: F, t60: F, t1184: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3430, t3431) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk667::<F>(t3426, t461, t221, t456, t1176, t135);
        let (t3433, t3436, t3439) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk668::<F>(t1179, t3431, t1174, t1186, t135, t1089, t405);
        let t3440 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk669::<F>(t3439, t974);
        let (t3441, t3442, t3443, t3447) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk670::<F>(t3242, t461, t2244, t3440, t337, t51, t1887);
        let t3448 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk671::<F>(t1176, t60);
        let t3449 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk672::<F>(t1184, t3448);
    (t3430, t3431, t3433, t3436, t3439, t3440, t3441, t3442, t3443, t3447, t3448, t3449)
}
