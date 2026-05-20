//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta643 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2189;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2190;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2191;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2192;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2193;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2194;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2195;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2196;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta643<F: Float>(t1880: F, t23196: F, t25224: F, t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t6552: F, t82124: F, t23237: F, t25341: F, t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F, t23270: F, t25038: F, t258: F, t4119: F, t776: F, t25039: F, t2553: F, t25040: F, t82074: F, t87712: F, t82294: F, t25193: F, t81591: F, t10049: F, t13053: F, t6632: F, t6663: F, t7538: F, t82296: F, t9593: F, t10109: F, t10110: F, t13029: F, t13042: F, t13050: F, t13058: F, t13071: F, t13460: F, t13463: F, t1528: F, t1902: F, t1911: F, t1912: F, t22979: F, t23191: F, t23215: F, t23278: F, t23281: F, t25168: F, t25169: F, t25170: F, t25188: F, t25200: F, t25233: F, t25329: F, t25330: F, t25348: F, t259: F, t2597: F, t2713: F, t2718: F, t2719: F, t2720: F, t2743: F, t4142: F, t4147: F, t4268: F, t4272: F, t4273: F, t4301: F, t47568: F, t47609: F, t6624: F, t6627: F, t6662: F, t7517: F, t7537: F, t82070: F, t82071: F, t82076: F, t82079: F, t82131: F, t82135: F, t82197: F, t82287: F, t855: F, t865: F, t866: F, t86905: F, t86909: F, t86911: F, t86916: F, t86923: F, t86952: F, t86955: F, t86961: F, t86968: F, t86972: F, t87005: F, t87010: F, t87013: F, t87741: F, t87792: F, t87797: F, t87805: F, t87806: F, t87807: F, t87827: F, t87836: F, t87837: F, t87847: F, t87880: F, t9590: F, t870: F, t1877: F, t1915: F, t22959: F, t23290: F, t25: F, t25013: F, t25021: F, t25024: F, t2522: F, t25377: F, t25381: F, t25392: F, t4314: F, t6666: F, t6670: F, t6671: F, t81483: F, t86803: F, t86806: F, t86810: F, t86816: F, t86821: F, t86825: F, t86830: F, t86835: F, t86836: F, t1484: F, t2249: F, t606: F, t1408: F, t2749: F, t10143: F, t7540: F, t13191: F, t25014: F, t13196: F, t13471: F, t25373: F, t57921: F, t1530: F, t16596: F, t81547: F, t22951: F, t22968: F, t23295: F, t23296: F, t23302: F, t25354: F, t25358: F, t6542: F, t7541: F, t86752: F, t86801: F, t25608: F, t381: F, t25428: F, t6712: F, t13797: F, t1926: F, t221: F, t10216: F, t387: F, t10277: F, t1625: F, t225: F, t344: F, t12648: F, t14165: F, t1927: F, t23327: F, t23329: F, t23332: F, t23588: F, t23594: F, t23728: F, t25416: F, t25423: F, t25425: F, t25429: F, t25431: F, t25432: F, t25442: F, t25815: F, t4548: F, t6691: F, t7553: F, t82402: F, t82417: F, t82502: F, t83352: F) -> (F, F, F, F, F, F, F) {
        let (t87893, t87898, t87902, t87904, t87907) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2189::<F>(t1880, t23196, t25224, t23030, t25205, t23164, t7479, t82133, t6552, t82124, t23237, t25341);
        let (t87911, t87915, t87920) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2190::<F>(t23204, t25216, t6562, t1519, t212, t23171, t6554, t23270, t25038, t258, t4119, t776);
        let t87940 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2191::<F>(t23270, t25038, t25039, t2553, t25040, t82074, t87712, t82294, t25193, t81591, t10049, t13053, t6632, t6663, t7538, t82296, t87915, t87920, t9593);
        let t87944 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2192::<F>(t10049, t10109, t10110, t13029, t13042, t13050, t13058, t13071, t13460, t13463, t1528, t1902, t1911, t1912, t22979, t23191, t23215, t23278, t23281, t25168, t25169, t25170, t25188, t25200, t25233, t25329, t25330, t25348, t259, t2597, t2713, t2718, t2719, t2720, t2743, t4142, t4147, t4268, t4272, t4273, t4301, t47568, t47609, t6624, t6627, t6632, t6662, t7517, t7537, t7538, t82070, t82071, t82076, t82079, t82131, t82135, t82197, t82287, t855, t865, t866, t86905, t86909, t86911, t86916, t86923, t86952, t86955, t86961, t86968, t86972, t87005, t87010, t87013, t87741, t87792, t87797, t87805, t87806, t87807, t87827, t87836, t87837, t87847, t87880, t87893, t87898, t87902, t87904, t87907, t87911, t87940, t9590);
        let (t87945, t87952) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2193::<F>(t870, t87944, t1877, t1915, t22959, t23290, t25, t25013, t25021, t25024, t2522, t25377, t25381, t25392, t4314, t6666, t6670, t6671, t81483, t86803, t86806, t86810, t86816, t86821, t86825, t86830, t86835, t86836);
        let (t87953, t87957, t87961, t87975, t87978, t87981, t87984) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2194::<F>(t1484, t2249, t4119, t606, t1408, t2749, t10143, t7540, t13191, t25014, t13196, t13471, t25);
        let t88001 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2195::<F>(t25373, t57921, t1530, t2249, t16596, t81547, t1877, t1915, t22951, t22959, t22968, t23295, t23296, t23302, t25013, t2522, t25354, t25358, t4314, t606, t6542, t6670, t7541, t87953, t87957, t87961, t87975, t87978, t87981, t87984);
        let (t88003, t88004, t88016, t88022, t88023) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2196::<F>(t86752, t86801, t87952, t88001, t25608, t381, t25428, t6712, t13797, t1926, t221, t10216, t387);
        let t88054 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2197::<F>(t10277, t387, t1625, t225, t344, t12648, t14165, t1927, t23327, t23329, t23332, t23588, t23594, t23728, t25416, t25423, t25425, t25429, t25431, t25432, t25442, t25815, t4548, t6691, t7553, t82402, t82417, t82502, t83352, t88004, t88016, t88022, t88023);
    (t87944, t87945, t87975, t88003, t88016, t88022, t88054)
}
