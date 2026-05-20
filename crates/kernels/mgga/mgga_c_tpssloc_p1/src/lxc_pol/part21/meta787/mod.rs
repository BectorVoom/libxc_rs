//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta787 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2738;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2739;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2740;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2741;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2742;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2743;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta787<F: Float>(t1799: F, t3698: F, t20063: F, t3701: F, t1388: F, t15899: F, t3918: F, t39642: F, t39655: F, t39658: F, t5160: F, t57206: F, t57207: F, t57209: F, t57210: F, t57212: F, t57213: F, t57214: F, t20085: F, t3914: F, t39844: F, t57215: F, t57216: F, t57218: F, t57219: F, t57220: F, t57221: F, t57222: F, t57223: F, t57224: F, t57225: F, t12470: F, t193: F, t3924: F, t40224: F, t40230: F, t56486: F, t57226: F, t57228: F, t57230: F, t57231: F, t57232: F, t57233: F, t57236: F, t57237: F, t6330: F, t40: F, t12606: F, t12652: F, t12862: F, t16549: F, t16554: F, t16558: F, t2244: F, t2250: F, t2433: F, t40632: F, t4080: F, t5392: F, t5398: F, t55677: F, t55723: F, t607: F, t73: F, t9427: F, zeta_threshold: F, t52: F, t12874: F, t16563: F, t16568: F, t2440: F, t40647: F, t4087: F, t76: F, t9438: F, t157: F, t182: F, t145: F, t185: F, t46125: F, t46128: F, t46130: F, t16576: F, t751: F, t46132: F, t16583: F, t16589: F, t39249: F, t39256: F, t40626: F, t46341: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t57810 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2738::<F>(t1799, t3698, t20063, t3701, t1388, t15899, t3918, t39642, t39655, t39658, t5160, t57206, t57207, t57209, t57210, t57212, t57213, t57214);
        let t57815 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2739::<F>(t20085, t3914, t39844, t5160, t57215, t57216, t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225);
        let t57822 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2740::<F>(t12470, t193, t3924, t40224, t40230, t56486, t57226, t57228, t57230, t57231, t57232, t57233, t57236, t57237, t6330);
        let t57850 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2741::<F>(t40, t12606, t12652, t12862, t16549, t16554, t16558, t2244, t2250, t2433, t40632, t4080, t5392, t5398, t55677, t55723, t607, t73, t9427, zeta_threshold);
        let t57873 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2742::<F>(t52, t12606, t12652, t12874, t16558, t16563, t16568, t2244, t2250, t2440, t40647, t4087, t5392, t5398, t55677, t55723, t607, t76, t9438, zeta_threshold);
        let (t57877, t57879, t57880, t57885, t57886, t57888, t57889) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2743::<F>(t57850, t57873, t157, t182, t145, t185, t46125, t46128, t46130, t16576, t751, t46132);
        let t57890 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2744::<F>(t16583, t16589, t39249, t39256, t40626, t46341, t57877, t57879, t57880, t57885, t57886, t57888, t57889);
    (t57810, t57815, t57822, t57877, t57879, t57880, t57885, t57886, t57888, t57889, t57890)
}
