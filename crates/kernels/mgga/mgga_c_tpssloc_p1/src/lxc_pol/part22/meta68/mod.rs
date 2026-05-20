//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta68 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk477;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk478;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk479;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk480;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk481;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk482;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk483;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta68<F: Float>(t1338: F, t562: F, t1352: F, t1372: F, t553: F, t1332: F, t1336: F, t544: F, t564: F, t1378: F, t1324: F, t1373: F, t1375: F, t568: F, t570: F, t1274: F, t1276: F, t1286: F, t1288: F, t1290: F, t1293: F, t1296: F, t1297: F, t1307: F, t193: F, t533: F, t680: F, t705: F, t113: F, t1266: F, t1271: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F, t3: F, t576: F, t112: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1380 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk477::<F>(t1338, t562);
        let (t1381, t1383, t1385) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk478::<F>(t1352, t1380, t1372, t553, t1332, t1336, t544, t564);
        let t1386 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk479::<F>(t1378, t1385);
        let t1388 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk480::<F>(t1324, t1373, t1375, t1386, t568);
        let t1390 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk481::<F>(t570);
        let t1393 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk482::<F>(t1274, t1276, t1286, t1288, t1290, t1293, t1296, t1297, t1307, t1388, t1390, t193, t533, t680, t705);
        let t1395 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk483::<F>(t113, t1266, t1271, t1393, t510, t513, t574, t650, t652, t672);
        let (t1396, t1398, t1401) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk484::<F>(t1395, t3, t576, t112);
    (t1380, t1381, t1383, t1385, t1386, t1388, t1390, t1393, t1395, t1396, t1398, t1401)
}
