//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta331 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1181;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1182;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1183;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1184;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1185;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1186;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1187;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1188;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1189;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1190;
use chunk10::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta331<F: Float>(t40419: F, t535: F, t9538: F, t12231: F, t3726: F, t12199: F, t12208: F, t118: F, t12012: F, t3739: F, t794: F, t12217: F, t40021: F, t1315: F, t210: F, t214: F, t39892: F, t40025: F, t40026: F, t40401: F, t40402: F, t40404: F, t40407: F, t40410: F, t40415: F, t40389: F, t225: F, t3774: F, t3862: F, t241: F, t6597: F, t248: F, t555: F, t557: F, t12368: F, t12369: F, t12402: F, t12407: F, t12419: F, t12420: F, t12422: F, t12426: F, t12429: F, t1352: F, t16233: F, t16305: F, t3803: F, t3805: F, t3807: F, t40183: F, t40197: F, t40304: F, t40329: F, t40335: F, t5246: F, t5248: F, t5250: F, t554: F, t559: F, t39970: F, t40010: F, t40062: F, t40101: F, t40147: F, t40204: F, t40303: F, t12167: F, t562: F, t12434: F, t1338: F, t3787: F, t3879: F, t12248: F, t1372: F, t12169: F, t12171: F, t12178: F, t12244: F, t12255: F, t12259: F, t12260: F, t12435: F, t1332: F, t1336: F, t22694: F, t3773: F, t3777: F, t3851: F, t3856: F, t3901: F, t3909: F, t5344: F, t544: F, t553: F, t40041: F, t12168: F, t12172: F, t12179: F, t12240: F, t12241: F, t12256: F, t12267: F, t12273: F, t1380: F, t16033: F, t16047: F, t16055: F, t22740: F, t3905: F, t40271: F, t5334: F, t564: F, t12019: F, t566: F, t68: F, t3888: F, t12023: F, t12027: F, t12030: F, t12033: F, t12181: F, t12237: F, t12238: F, t12249: F, t12251: F, t12252: F, t12438: F, t1323: F, t1375: F, t1378: F, t1381: F, t1383: F, t3752: F, t3758: F, t3793: F, t3882: F, t3889: F, t3897: F, t3898: F, t3902: F, t3907: F, t39938: F, t40047: F, t40118: F, t40133: F, t40148: F, t40153: F, t40162: F, t539: F, t568: F, t3698: F, t3700: F, t1297: F, t1390: F, t16490: F, t193: F, t3719: F, t39852: F, t39854: F, t39856: F, t39858: F, t39932: F, t40222: F, t40224: F, t40226: F, t40228: F, t40230: F, t40232: F, t40234: F, t533: F, t40: F, t10121: F, t870: F, t2517: F, t2519: F, t195: F, t632: F, t2244: F, t2250: F, t2433: F, t39097: F, t39103: F, t39110: F, t73: F, t9258: F, t9427: F, t9430: F, zeta_threshold: F, t52: F, t197: F, t636: F, t2440: F, t76: F, t9438: F, t9441: F, t145: F, t185: F, t2531: F, t9892: F, t67: F, t758: F, t9915: F, t10126: F, t2379: F, t2522: F, t2523: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t4314: F, t776: F, t9516: F) -> (F, F, F, F, F, F, F, F) {
        let (t40422, t40423, t40425, t40429, t40431) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1181::<F>(t40419, t535, t9538, t12231, t3726, t12199, t12208, t118, t12012, t3739, t794, t12217, t40021);
        let t40437 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1182::<F>(t1315, t210, t214, t39892, t40025, t40026, t40401, t40402, t40404, t40407, t40410, t40415, t40422, t40423, t40425, t40429, t40431);
        let (t40438, t40439, t40445, t40450) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1183::<F>(t40389, t40437, t225, t3774, t3862, t241, t6597, t248, t555, t557, t12368, t12369, t12402, t12407, t12419, t12420, t12422, t12426, t12429, t1352, t16233, t16305, t3803, t3805, t3807, t40183, t40197, t40304, t40329, t40335, t5246, t5248, t5250, t554, t559);
        let (t40453, t40475, t40479) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1184::<F>(t39970, t40010, t40062, t40101, t40147, t40204, t40303, t40450, t12167, t562, t12434, t1338);
        let (t40486, t40492, t40524) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1185::<F>(t3787, t3879, t12248, t1372, t12169, t12171, t12178, t12244, t12255, t12259, t12260, t12435, t1332, t1336, t1352, t22694, t3773, t3777, t3851, t3856, t3901, t3909, t40453, t40475, t5344, t544, t553);
        let (t40541, t40576) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1186::<F>(t40041, t562, t12168, t12172, t12179, t12240, t12241, t12256, t12267, t12273, t1336, t1380, t16033, t16047, t16055, t22740, t3777, t3901, t3905, t40271, t40335, t40439, t5334, t564);
        let t40603 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1187::<F>(t12019, t566, t68, t3888, t12023, t12027, t12030, t12033, t12181, t12237, t12238, t12240, t12249, t12251, t12252, t12259, t12267, t12434, t12438, t1323, t1336, t1352, t1372, t1375, t1378, t1380, t1381, t1383, t22694, t22740, t3752, t3758, t3777, t3793, t3851, t3879, t3882, t3889, t3897, t3898, t3902, t3907, t39938, t40047, t40118, t40133, t40148, t40153, t40162, t40438, t40453, t40475, t40479, t40486, t40492, t40524, t40541, t40576, t5250, t5334, t5344, t539, t562, t568);
        let t40615 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1188::<F>(t3698, t3700, t1297, t1390, t16490, t193, t3719, t39852, t39854, t39856, t39858, t39892, t39932, t40222, t40224, t40226, t40228, t40230, t40232, t40234, t40603, t533);
        let (t40622, t40627, t40645) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1189::<F>(t40, t10121, t870, t2517, t2519, t195, t632, t2244, t2250, t2433, t39097, t39103, t39110, t73, t9258, t9427, t9430, zeta_threshold);
        let t40660 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1190::<F>(t52, t197, t636, t2244, t2250, t2440, t39097, t39103, t39110, t76, t9258, t9438, t9441, zeta_threshold);
        let (t40661, t40663, t40668, t40671, t40672) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1191::<F>(t40645, t40660, t145, t185, t2531, t9892, t67, t758, t9915, t10126, t2379, t2522, t2523, t39249, t39256, t39309, t39312, t39316, t39320, t40622, t40627, t4314, t776, t9516);
    (t40445, t40615, t40627, t40661, t40663, t40668, t40671, t40672)
}
