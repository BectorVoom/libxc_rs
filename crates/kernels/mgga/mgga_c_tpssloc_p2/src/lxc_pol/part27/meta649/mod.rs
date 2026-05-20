//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta649 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2250;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2251;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2252;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2253;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2254;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2255;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2256;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2257;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2258;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2259;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2260;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta649<F: Float>(t25836: F, t3216: F, t11094: F, t7627: F, t1068: F, t1070: F, t14662: F, t1637: F, t193: F, t23738: F, t23742: F, t25840: F, t25845: F, t3209: F, t3213: F, t336: F, t4696: F, t4700: F, t60941: F, t6822: F, t83468: F, t83472: F, t83479: F, t88054: F, t88097: F, t88137: F, t88179: F, t88213: F, t88742: F, t88779: F, t88827: F, t88867: F, t88900: F, t88940: F, t89556: F, t89590: F, t89623: F, t89658: F, t89690: F, t12915: F, t13487: F, t13191: F, t13471: F, t1530: F, t16596: F, t1877: F, t1915: F, t202: F, t22959: F, t23290: F, t2379: F, t25013: F, t2522: F, t25358: F, t25365: F, t25374: F, t2553: F, t4119: F, t4314: F, t57893: F, t57912: F, t6666: F, t6670: F, t7541: F, t81525: F, t81539: F, t82312: F, t86717: F, t868: F, t86836: F, t870: F, t87944: F, t12971: F, t13196: F, t1484: F, t23286: F, t23295: F, t25354: F, t2745: F, t2749: F, t4255: F, t4303: F, t47645: F, t57921: F, t58009: F, t58071: F, t59580: F, t7634: F, t776: F, t86706: F, t86713: F, t86815: F, t87975: F, t25: F, t265: F, t394: F, t12606: F, t1409: F, t1965: F, t2250: F, t23773: F, t25883: F, t3966: F, t40: F, t607: F, t6835: F, t7643: F, t88003: F, dens_threshold: F, rho0: F, zeta_threshold: F, t23788: F, t25891: F, t25927: F, t10143: F, t1081: F, t1649: F, t23789: F, t23792: F, t25372: F, t6848: F, t86736: F, t28: F, t3231: F, t86781: F, t25928: F, t25945: F, t7649: F, t86703: F, t86734: F, t86751: F, t86757: F, t87945: F, t86797: F, t83555: F, t40772: F, t23781: F, t23807: F, t23810: F, t23813: F, t25892: F, t25898: F, t25905: F, t6841: F, t81483: F, t86740: F, t86775: F, t86835: F, t2752: F, t23796: F, t25901: F, t25921: F, t25930: F, t25934: F, t25938: F, t7650: F, t7656: F, t504: F, t1972: F, t23821: F, t25950: F, t52: F, t6856: F, t7664: F, rho1: F, t23858: F, t7685: F, t22607: F, t7688: F, t1390: F, t16018: F, t1983: F, t6878: F, t22574: F, t56194: F, t8643: F, t113: F, t1393: F, t1459: F, t26138: F, t4072: F, t5107: F, t6515: F, t652: F, t6862: F, t83935: F, t86673: F, t86676: F, t86679: F, t86682: F, t86684: F, t86688: F, t86693: F, t86698: F, t86700: F, t86702: F) -> F {
        let t89729 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2250::<F>(t25836, t3216, t11094, t7627, t1068, t1070, t14662, t1637, t193, t23738, t23742, t25840, t25845, t3209, t3213, t336, t4696, t4700, t60941, t6822, t83468, t83472, t83479, t88054, t88097, t88137, t88179, t88213, t88742, t88779, t88827, t88867, t88900, t88940, t89556, t89590, t89623, t89658, t89690);
        let t89775 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2251::<F>(t12915, t13487, t13191, t13471, t1530, t16596, t1877, t1915, t193, t202, t22959, t23290, t2379, t25013, t2522, t25358, t25365, t25374, t2553, t4119, t4314, t57893, t57912, t6666, t6670, t7541, t81525, t81539, t82312, t86717, t868, t86836, t870, t87944);
        let t89822 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2252::<F>(t12971, t13196, t1484, t1877, t1915, t23286, t23290, t23295, t2522, t25354, t25358, t2745, t2749, t4255, t4303, t4314, t47645, t57921, t58009, t58071, t59580, t6666, t6670, t7634, t776, t86706, t86713, t86815, t87975);
        let (t89823, t89836) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2253::<F>(t25, t265, t394, t89775, t89822, t89729, t12606, t1409, t1965, t2250, t23773, t25883, t3966, t40, t607, t6835, t7643, t88003, dens_threshold, rho0, zeta_threshold);
        let (t89837, t89840, t89843, t89846, t89850, t89859) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2254::<F>(t23788, t59580, t86815, t13196, t25891, t25927, t58009, t10143, t1081, t25374, t4255, t870);
        let t89880 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2255::<F>(t23788, t58071, t86706, t1649, t2745, t25927, t86713, t2379, t1877, t1915, t22959, t23789, t23792, t25013, t2522, t25372, t4314, t6670, t6848, t7541, t86736, t86836, t89837, t89840, t89843, t89846, t89850, t89859);
        let (t89881, t89888, t89892, t89896, t89904, t89907, t89911) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2256::<F>(t1649, t2553, t12971, t28, t1081, t4119, t13191, t25891, t25927, t57921, t13471, t1484, t3231);
        let t89920 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2257::<F>(t25927, t86781, t1877, t1915, t22959, t23286, t23290, t25013, t2522, t25928, t25945, t28, t6670, t7649, t86703, t86734, t86751, t86757, t87945, t89881, t89888, t89892, t89896, t89904, t89907, t89911);
        let t89957 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2258::<F>(t23788, t86797, t16596, t83555, t1081, t4303, t28, t40772, t86717, t1877, t22959, t23781, t23807, t23810, t23813, t25013, t2522, t25354, t25358, t25372, t25892, t25898, t25905, t4314, t6666, t6670, t6841, t7541, t81483, t86740, t86775, t86835, t87975);
        let t90001 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2259::<F>(t25365, t83555, t1530, t3231, t1649, t2749, t23788, t57893, t2752, t13487, t1877, t22959, t23286, t23290, t23295, t23796, t2522, t25901, t25921, t25930, t25934, t25938, t47645, t6666, t6670, t7541, t7650, t7656, t81483, t81525);
        let t90016 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2260::<F>(t28, t265, t504, t89880, t89920, t89957, t90001, t89823, t12606, t1409, t1972, t2250, t23821, t25950, t3966, t52, t607, t6856, t7664, dens_threshold, rho1, zeta_threshold);
        let t90030 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2261::<F>(t23858, t7685, t22607, t7688, t1390, t16018, t1983, t6878, t22574, t56194, t8643, t113, t1393, t1459, t26138, t4072, t5107, t6515, t652, t6862, t83935, t86673, t86676, t86679, t86682, t86684, t86688, t86693, t86698, t86700, t86702, t89836, t90016);
    t90030
}
