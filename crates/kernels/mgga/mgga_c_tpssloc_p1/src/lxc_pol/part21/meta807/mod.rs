//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta807 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2810;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2811;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2812;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2813;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2814;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2815;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2816;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2817;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2818;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2819;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2820;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta807<F: Float>(t2627: F, t5631: F, t13176: F, t13417: F, t13431: F, t13434: F, t1523: F, t16823: F, t17028: F, t255: F, t2617: F, t2633: F, t4162: F, t4166: F, t4296: F, t4298: F, t46528: F, t5648: F, t5653: F, t59074: F, t59230: F, t812: F, t860: F, t9612: F, t10076: F, t13385: F, t13390: F, t13401: F, t13404: F, t13429: F, t16753: F, t16759: F, t16811: F, t16815: F, t17027: F, t17034: F, t2684: F, t2732: F, t2740: F, t4182: F, t4281: F, t4291: F, t5575: F, t5617: F, t58226: F, t58262: F, t59331: F, t829: F, t4300: F, t10049: F, t10110: F, t13053: F, t13059: F, t13461: F, t1528: F, t17050: F, t17064: F, t17070: F, t17090: F, t2597: F, t2718: F, t2719: F, t2743: F, t4147: F, t4268: F, t4273: F, t4301: F, t47609: F, t47618: F, t5657: F, t5658: F, t58143: F, t58194: F, t58224: F, t58261: F, t58304: F, t58337: F, t59351: F, t855: F, t858: F, t866: F, t9590: F, t17100: F, t225: F, t13029: F, t13042: F, t13065: F, t13072: F, t1519: F, t16804: F, t17057: F, t17092: F, t259: F, t2591: F, t2713: F, t2720: F, t2742: F, t4142: F, t4265: F, t47568: F, t5636: F, t5637: F, t852: F, t9593: F, t17087: F, t17060: F, t13050: F, t13071: F, t13377: F, t13460: F, t13463: F, t1492: F, t1527: F, t17022: F, t17049: F, t25168: F, t46452: F, t46488: F, t47585: F, t798: F, t865: F, t17095: F, t17098: F, t17052: F, t218: F, t252: F, t2710: F, t40890: F, t46508: F, t5558: F, t59229: F, t59328: F, t10143: F, t5660: F, t12895: F, t1877: F, t193: F, t202: F, t2522: F, t2749: F, t39585: F, t39590: F, t39593: F, t4119: F, t58139: F, t58973: F, t58974: F, t58975: F, t58978: F, t58979: F, t58980: F, t766: F, t870: F, t1484: F, t2745: F, t17109: F, t2752: F, t13471: F, t16662: F, t17116: F, t262: F, t41254: F, t41258: F, t41262: F, t4307: F, t4314: F, t58983: F, t58985: F, t58986: F, t58987: F, t776: F, t868: F, t39658: F, t58988: F, t58989: F, t58990: F, t58991: F, t58993: F, t58996: F, t58999: F, t59001: F, t59005: F, t59008: F, t59009: F, t59011: F, t16625: F, t2379: F, t47645: F, t5502: F, t5544: F, t59014: F, t59015: F, t59016: F, t59018: F, t59019: F, t59020: F, t59023: F, t59025: F, t59027: F, t9470: F, t4303: F, t2553: F, t5527: F, t59029: F, t59031: F, t59033: F, t59034: F, t59035: F, t59038: F, t59040: F, t59043: F, t59046: F, t59049: F, t57890: F, t57901: F, t57931: F, t57955: F, t57976: F, t57994: F, t58024: F, t58036: F, t58054: F, t58063: F, t58095: F) -> F {
        let t59379 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2810::<F>(t2627, t5631, t13176, t13417, t13431, t13434, t1523, t16823, t17028, t255, t2617, t2633, t4162, t4166, t4296, t4298, t46528, t5648, t5653, t59074, t59230, t812, t860, t9612);
        let t59412 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2811::<F>(t10076, t13385, t13390, t13401, t13404, t13429, t16753, t16759, t16811, t16815, t17027, t17034, t2617, t2633, t2684, t2732, t2740, t4166, t4182, t4281, t4291, t5575, t5617, t58226, t58262, t59331, t812, t829);
        let t59434 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2812::<F>(t4300, t10049, t10110, t13053, t13059, t13461, t1528, t17050, t17064, t17070, t17090, t2597, t2718, t2719, t2743, t4147, t4268, t4273, t4301, t47609, t47618, t5657, t5658, t58143, t58194, t58224, t58261, t58304, t58337, t59351, t59379, t59412, t855, t858, t866, t9590);
        let t59475 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2813::<F>(t17100, t225, t10110, t13029, t13042, t13065, t13072, t13461, t1519, t1528, t16804, t17057, t17070, t17092, t259, t2591, t2713, t2720, t2742, t4142, t4147, t4265, t4273, t4301, t47568, t5631, t5636, t5637, t5658, t852, t855, t866, t9590, t9593);
        let t59518 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2814::<F>(t17087, t225, t17060, t13050, t13071, t13072, t13377, t13460, t13463, t1492, t1527, t1528, t17022, t17049, t17050, t17057, t25168, t259, t2597, t2713, t2718, t4147, t4268, t4273, t46452, t46488, t47585, t5637, t798, t855, t865, t866, t9593);
        let t59558 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2815::<F>(t17095, t225, t17098, t10049, t13042, t13059, t13463, t1528, t17052, t17064, t17090, t17092, t218, t252, t259, t2710, t2713, t2718, t2719, t2720, t2742, t2743, t40890, t4268, t4273, t4301, t46508, t5558, t5636, t5637, t5657, t59229, t59328, t855, t866);
        let t59571 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2816::<F>(t10143, t5660, t12895, t1877, t193, t202, t2522, t2749, t39585, t39590, t39593, t4119, t58139, t58973, t58974, t58975, t58978, t58979, t58980, t59434, t59475, t59518, t59558, t766, t870);
        let t59591 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2817::<F>(t1484, t2745, t17109, t2752, t13471, t16662, t17116, t1877, t2522, t262, t41254, t41258, t41262, t4307, t4314, t58983, t58985, t58986, t58987, t776, t868);
        let t59592 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2818::<F>(t39658, t58988, t58989, t58990, t58991, t58993, t58996, t58999, t59001, t59005, t59008, t59009, t59011);
        let t59602 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2819::<F>(t16625, t2379, t2522, t4314, t47645, t5502, t5544, t59014, t59015, t59016, t59018, t59019, t59020, t59023, t59025, t59027, t9470);
        let t59614 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2820::<F>(t4303, t16625, t193, t202, t2522, t2553, t2752, t4314, t5527, t59029, t59031, t59033, t59034, t59035, t59038, t59040, t59043, t59046, t59049, t9470);
        let t59618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2821::<F>(t57890, t57901, t57931, t57955, t57976, t57994, t58024, t58036, t58054, t58063, t58095, t59571, t59591, t59592, t59602, t59614);
    t59618
}
