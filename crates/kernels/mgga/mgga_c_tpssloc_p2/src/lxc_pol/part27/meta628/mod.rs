//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta628 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2113;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2114;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2115;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2116;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta628<F: Float>(t25373: F, t86713: F, t25: F, t40772: F, t1530: F, t2749: F, t1408: F, t2752: F, t13487: F, t22960: F, t58071: F, t2: F, t584: F, t868: F, t25372: F, t193: F, t201: F, t7540: F, t200: F, t6665: F, t4303: F, t606: F, t1877: F, t1915: F, t9212: F, t22959: F, t22961: F, t25013: F, t25015: F, t2522: F, t25366: F, t25375: F, t25385: F, t6666: F, t6670: F, t81483: F, t86703: F, t86707: F, t86710: F, t870: F, t776: F, t2553: F, t10143: F, t25374: F, t25365: F, t58009: F, t4255: F, t2249: F, t22964: F, t23286: F, t23299: F, t25028: F, t25358: F, t47645: F, t7475: F, t7476: F, t7541: F, t7545: F, t81525: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t86714, t86717, t86718, t86722, t86727, t86730) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2113::<F>(t25373, t86713, t25, t40772, t1530, t2749, t1408, t2752, t13487, t22960, t58071, t2);
        let (t86734, t86736, t86740, t86746, t86751) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2114::<F>(t584, t86730, t868, t25372, t193, t201, t7540, t200, t6665, t4303, t606, t1877, t1915, t9212);
        let t86752 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2115::<F>(t1877, t22959, t22961, t25013, t25015, t2522, t25366, t25372, t25375, t25385, t6666, t6670, t81483, t86703, t86707, t86710, t86714, t86718, t86722, t86727, t86734, t86736, t86740, t86746, t86751);
        let (t86757, t86764, t86771, t86775) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2116::<F>(t2, t870, t584, t776, t22959, t1408, t2553, t10143, t606, t25374, t1877, t1915);
        let (t86781, t86797, t86801) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2117::<F>(t25365, t868, t25373, t58009, t4255, t22960, t1408, t1877, t1915, t2249, t22959, t22964, t23286, t23299, t25013, t25028, t2522, t25358, t25372, t47645, t6666, t7475, t7476, t7541, t7545, t81525, t86757, t86764, t86771, t86775);
    (t86717, t86734, t86736, t86740, t86751, t86752, t86757, t86775, t86781, t86797, t86801)
}
