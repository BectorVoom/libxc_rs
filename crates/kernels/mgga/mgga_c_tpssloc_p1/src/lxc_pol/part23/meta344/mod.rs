//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1130;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1131;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta344<F: Float>(t2225: F, t3824: F, t1287: F, t9214: F, t39033: F, t522: F, t39035: F, t39031: F, t16: F, t185: F, t520: F, t9212: F, t9218: F, t118: F, t142: F, t39283: F, t2223: F, t2475: F, t2461: F, t2478: F, t159: F, t172: F, t2454: F, t268: F, t39249: F, t39256: F, t39300: F, t39309: F, t39312: F, t39316: F, t39320: F, t39377: F, t39378: F, t39381: F, t39535: F, t676: F, t724: F, t732: F, t739: F, t740: F, t746: F, t747: F, t781: F, t9493: F, t9720: F, t9738: F, t9740: F, t9752: F, t9762: F, t9763: F, t9781: F, t9828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39595, t39597, t39604, t39606, t39608, t39615, t39634) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1130::<F>(t2225, t3824, t1287, t9214, t39033, t522, t39035, t39031, t16, t185, t520, t9212);
        let (t39635, t39655, t39658) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1131::<F>(t39634, t1287, t9218, t118, t142, t39283);
        let (t39660, t39664, t39706) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1132::<F>(t2223, t3824, t2475, t2461, t2478, t159, t172, t2454, t268, t39249, t39256, t39300, t39309, t39312, t39316, t39320, t39377, t39378, t39381, t39535, t676, t724, t732, t739, t740, t746, t747, t781, t9493, t9720, t9738, t9740, t9752, t9762, t9763, t9781, t9828);
    (t39595, t39597, t39604, t39606, t39608, t39615, t39635, t39655, t39658, t39660, t39664, t39706)
}
