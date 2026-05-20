//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta286 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1333;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1334;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1335;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1336;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1337;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1338;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1339;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta286<F: Float>(t731: F, t9751: F, t746: F, t9490: F, t172: F, t9489: F, t9493: F, t9720: F, t2512: F, t9711: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F, t702: F, t683: F, t9731: F, t2405: F, t2420: F, t703: F, t204: F, t682: F, t268: F, t2419: F, t2421: F, t676: F, t118: F, t168: F, t2477: F, t2510: F, t725: F, t740: F, t9457: F, t9476: F, t9484: F, t9697: F, t9730: F, t9734: F, t9739: F, t9740: F, t2368: F, t739: F, t2509: F, t724: F, t2406: F, t2483: F, t2410: F, t2415: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9777) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1333::<F>(t731, t9751, t746, t9490, t172, t9489, t9493, t9720, t2512, t9711, t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
        let t9780 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1334::<F>(t702, t9777, t683);
        let (t9781, t9789) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1335::<F>(t731, t9731, t2405, t2420, t703);
        let t9793 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1336::<F>(t204, t682, t268, t703);
        let t9797 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1337::<F>(t2419, t2421, t268, t676);
        let t9798 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1338::<F>(t118, t168, t2477, t2510, t725, t740, t9457, t9476, t9484, t9697, t9730, t9734, t9739, t9740, t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9780, t9781, t9789, t9793, t9797);
        let (t9799, t9803, t9810, t9814, t9820) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1339::<F>(t2368, t676, t204, t739, t2509, t724, t2406, t2483, t268);
        let t9824 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1340::<F>(t2410, t676, t2415, t268);
    (t9780, t9789, t9793, t9797, t9798, t9799, t9803, t9810, t9814, t9820, t9824)
}
