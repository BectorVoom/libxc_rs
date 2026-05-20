//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta120 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk685;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk686;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk687;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk688;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk689;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta120<F: Float>(t865: F, t2718: F, t252: F, t2627: F, t2633: F, t814: F, t852: F, t829: F, t2679: F, t860: F, t2684: F, t235: F, t2710: F, t226: F, t255: F, t2613: F, t2617: F, t808: F, t812: F, t861: F, t863: F, t858: F, t259: F, t2592: F, t2594: F, t2597: F, t2711: F, t2713: F, t855: F, t866: F, t868: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2719 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk685::<F>(t865);
        let t2720 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk686::<F>(t2718, t2719);
        let (t2729, t2732, t2733, t2736, t2738, t2740, t2742) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk687::<F>(t252, t2627, t2633, t814, t852, t829, t2679, t860, t2684, t235, t2710, t226, t255, t2613, t2617, t808, t812, t861, t863);
        let t2743 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk688::<F>(t2742, t858);
        let t2745 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk689::<F>(t259, t2592, t2594, t2597, t2711, t2713, t2720, t2743, t855, t866);
        let t2749 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk690::<F>(t868);
    (t2719, t2720, t2729, t2732, t2733, t2736, t2738, t2740, t2742, t2743, t2745, t2749)
}
