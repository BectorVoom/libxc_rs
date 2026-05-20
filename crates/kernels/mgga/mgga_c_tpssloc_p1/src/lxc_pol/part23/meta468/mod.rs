//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta468 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1374;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1375;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1376;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1377;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1378;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1379;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1380;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1381;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1382;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1383;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1384;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1385;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta468<F: Float>(t2792: F, t76998: F, t913: F, t10632: F, t41825: F, t76637: F, t959: F, t5742: F, t48103: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t68494: F, t68498: F, t68500: F, t77028: F, t77030: F, t77032: F, t77034: F, t59657: F, t60168: F, t60173: F, t60204: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t77042: F, t77073: F, t77076: F, t42212: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F, t42213: F, t47787: F, t76587: F, t76595: F, t76610: F, t76618: F, t76626: F, t76899: F, t76903: F, t76906: F, t76912: F, t77102: F, t77105: F, t77107: F, t5758: F, t10756: F, t10771: F, t10813: F, t10828: F, t14337: F, t17355: F, t21198: F, t21207: F, t21239: F, t21242: F, t21247: F, t2886: F, t2888: F, t2905: F, t2932: F, t42111: F, t42113: F, t42154: F, t42226: F, t42228: F, t4449: F, t49099: F, t49104: F, t49285: F, t5775: F, t5791: F, t5794: F, t60343: F, t60424: F, t77139: F, t924: F, t932: F, t951: F, t10811: F, t1569: F, t1581: F, t17428: F, t21115: F, t21195: F, t2861: F, t41826: F, t4411: F, t49430: F, t5743: F, t5759: F, t5762: F, t59920: F, t60407: F, t69047: F, t69182: F, t76647: F, t76652: F, t76654: F, t76657: F, t76659: F, t76661: F, t77220: F, t943: F, t14271: F, t1568: F, t17499: F, t17547: F, t21194: F, t21306: F, t5790: F, t69380: F, t76632: F, t76663: F, t76665: F, t76668: F, t76671: F, t77001: F, t77006: F, t77119: F, t77124: F, t77127: F, t77130: F, t42245: F, t41655: F, t291: F, t14263: F, t14276: F, t1580: F, t21238: F, t21259: F, t21309: F, t21312: F, t21321: F, t2930: F, t311: F, t59941: F, t69276: F, t77133: F, t77135: F, t77138: F, t77226: F, t77229: F, t300: F, t10629: F, t2929: F, t77153: F, t77157: F, t77159: F, t77224: F, t76995: F, t77017: F, t77151: F, t1020: F, t1021: F, t1041: F, t1044: F, t1618: F, t17607: F, t21580: F, t248: F, t3062: F, t3131: F, t360: F, t369: F, t378: F, t42347: F, t43317: F, t4644: F, t5880: F, t5900: F, t61739: F, t68: F, t70148: F, t70162: F, t70166: F, t70199: F, t70209: F, t70214: F, t70227: F, t75836: F, t76597: F, t76612: F, t76620: F, t76740: F, t76977: F, t973: F, t974: F) -> (F, F, F, F, F, F, F, F) {
        let (t77232, t77236, t77239, t77257) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1374::<F>(t2792, t76998, t913, t10632, t41825, t76637, t959, t5742, t48103, t68442, t68444, t68446, t68448, t68452, t68454, t68494, t68498, t68500, t77028, t77030, t77032, t77034);
        let t77272 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1375::<F>(t59657, t60168, t60173, t60204, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t77042, t77073, t77076);
        let t77287 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1376::<F>(t42212, t59688, t59694, t76574, t76578, t76583, t76591, t76599, t76614, t76622, t76893, t76896, t76909, t76915);
        let t77301 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1377::<F>(t42213, t47787, t76587, t76595, t76610, t76618, t76626, t76899, t76903, t76906, t76912, t77102, t77105, t77107);
        let (t77328, t77343) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1378::<F>(t5758, t10756, t10771, t10813, t10828, t14337, t17355, t21198, t21207, t21239, t21242, t21247, t2886, t2888, t2905, t2932, t42111, t42113, t42154, t42226, t42228, t4449, t49099, t49104, t49285, t5775, t5791, t5794, t60343, t60424, t76637, t77139, t77239, t77257, t77272, t77287, t77301, t924, t932, t951);
        let t77370 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1379::<F>(t10632, t10811, t1569, t1581, t17428, t21115, t21195, t2861, t2888, t41826, t4411, t49430, t5743, t5759, t5762, t59920, t60407, t69047, t69182, t76637, t76647, t76652, t76654, t76657, t76659, t76661, t77220, t77239, t77328, t932, t943, t951);
        let t77390 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1380::<F>(t10756, t10771, t14271, t1568, t1569, t17499, t17547, t21194, t21306, t2861, t2886, t5742, t5743, t5758, t5790, t69380, t76632, t76663, t76665, t76668, t76671, t77001, t77006, t77119, t77124, t77127, t77130);
        let (t77427, t77440) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1381::<F>(t42245, t47787, t59657, t68442, t76574, t76578, t76583, t76587, t76591, t76595, t76599, t59688, t59694, t68444, t68446, t68448, t68494, t68498, t76610, t76614, t76618, t76622, t76626);
        let (t77454, t77467) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1382::<F>(t41655, t47787, t59657, t68442, t76574, t76578, t76583, t76587, t76591, t76595, t76599, t59688, t59694, t68444, t68446, t68448, t68494, t68498, t76610, t76614, t76618, t76622, t76626);
        let (t77470, t77471) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1383::<F>(t291, t77454, t77467, t10811, t10828, t14263, t14271, t14276, t14337, t1580, t1581, t21238, t21259, t21309, t21312, t21321, t2905, t2930, t2932, t311, t5742, t5775, t5790, t5794, t59941, t69276, t77133, t77135, t77138, t77139, t77226, t77229, t77232, t77427, t77440);
        let (t77474, t77478, t77482, t77483) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1384::<F>(t300, t77343, t77370, t77390, t77471, t10629, t2932, t76637, t959, t2929, t77139, t77153, t77157, t77159, t77224, t77226, t77229, t77232, t77236, t77470);
        let (t77485, t77498) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1385::<F>(t76995, t77017, t77151, t77483, t1020, t1021, t1041, t1044, t1618, t17607, t21580, t248, t3062, t3131, t360, t369, t378, t42347, t43317, t4644, t5880, t5900, t61739, t68, t70148, t70162, t70166, t70199, t70209, t70214, t70227, t75836, t76597, t76612, t76620, t76740, t76977, t973, t974);
    (t77232, t77236, t77470, t77474, t77478, t77482, t77485, t77498)
}
