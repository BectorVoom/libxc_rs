//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta669 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2363;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2364;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2365;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2366;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2367;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2368;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2369;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2370;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta669<F: Float>(t1983: F, t2019: F, t55169: F, t510: F, t652: F, t86604: F, t26114: F, t6535: F, t26179: F, t2314: F, t25994: F, t12823: F, t7461: F, t12550: F, t1442: F, t22461: F, t22619: F, t23829: F, t26103: F, t4028: F, t4073: F, t6517: F, t7472: F, t90351: F, t91713: F, t91715: F, t91718: F, t91722: F, t91724: F, t91726: F, t9348: F, t25980: F, t4034: F, t12813: F, t89: F, t1874: F, t6525: F, t22561: F, t7458: F, t3652: F, t7467: F, t22579: F, t7685: F, t55934: F, t12725: F, t26168: F, t6876: F, t25989: F, t83886: F, t15857: F, t1873: F, t45632: F, t12841: F, t1774: F, t1849: F, t22559: F, t2320: F, t23855: F, t4037: F, t7670: F, t90352: F, t90030: F, t90422: F, t91574: F, t91617: F, t91663: F, t91709: F, t26135: F, t3941: F, t671: F, t2363: F, t2022: F, t2319: F, t1458: F, t16538: F, t16541: F, t23877: F, t23880: F, t26523: F, t4072: F, t5376: F, t577: F, t83980: F, t86642: F, t86646: F, t86647: F, t86651: F, t86653: F, t86655: F, t86656: F, t86660: F, t86668: F, t5381: F, t26509: F, t580: F, t1404: F, t7758: F, t1395: F, t7774: F, t1396: F, t1398: F, t26510: F, t26555: F, t3: F, t3932: F, t5364: F, t7020: F, t80599: F, t80601: F, t80605: F, t86640: F, t86580: F) -> F {
        let (t91730, t91735, t91737, t91739, t91747, t91749) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2363::<F>(t1983, t2019, t55169, t510, t652, t86604, t26114, t6535, t26179, t2314, t25994, t12823, t7461);
        let t91750 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2364::<F>(t12550, t1442, t22461, t22619, t23829, t26103, t4028, t4073, t510, t6517, t7472, t90351, t91713, t91715, t91718, t91722, t91724, t91726, t91730, t91735, t91737, t91739, t91747, t91749, t9348);
        let (t91752, t91755, t91757, t91759, t91762) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2365::<F>(t25980, t4034, t12813, t89, t1874, t26179, t6525, t22561, t7458, t3652, t652, t7467);
        let (t91763, t91765, t91767, t91769, t91771, t91777) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2366::<F>(t22579, t7685, t1874, t55934, t12725, t6525, t26168, t6876, t25989, t83886, t25994, t4034);
        let t91789 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2367::<F>(t15857, t1873, t652, t1874, t45632, t12841, t1774, t1849, t22461, t22559, t2320, t23855, t4037, t510, t6517, t7670, t90352, t91752, t91755, t91757, t91759, t91762, t91763, t91765, t91767, t91769, t91771, t91777);
        let (t91792, t91799, t91802) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2368::<F>(t90030, t90422, t91574, t91617, t91663, t91709, t91750, t91789, t26135, t3941, t671, t2363, t7467);
        let t91806 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2369::<F>(t2022, t2319, t1458, t16538, t16541, t2363, t23877, t23880, t26523, t4072, t5376, t577, t671, t83980, t86642, t86646, t86647, t86651, t86653, t86655, t86656, t86660, t86668, t91792, t91799, t91802);
        let t91827 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2370::<F>(t2022, t5381, t26509, t580, t1404, t7758, t1395, t7774, t1396, t1398, t26510, t26555, t3, t3932, t5364, t7020, t80599, t80601, t80605, t86640, t91792, t91806);
        let tv4rho3sigma3 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2371::<F>(t86580, t91827);
    tv4rho3sigma3
}
