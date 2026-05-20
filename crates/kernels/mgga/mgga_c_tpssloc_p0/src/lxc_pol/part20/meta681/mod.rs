//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta681 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2567;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2568;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2569;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2570;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2571;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2572;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta681<F: Float>(t1254: F, t3633: F, t1157: F, t1164: F, t14829: F, t3375: F, t14966: F, t3378: F, t15823: F, t225: F, t15800: F, t15808: F, t11598: F, t11919: F, t11935: F, t1238: F, t1251: F, t1252: F, t14972: F, t15786: F, t15794: F, t15797: F, t15803: F, t15820: F, t1751: F, t1761: F, t3487: F, t3598: F, t3600: F, t3631: F, t44412: F, t4945: F, t498: F, t14731: F, t15419: F, t3447: F, t12606: F, t3450: F, t1714: F, t44583: F, t3451: F, t458: F, t44584: F, t4904: F, t44510: F, t14753: F, t15402: F, t11509: F, t11566: F, t11576: F, t11580: F, t11585: F, t11594: F, t15376: F, t15395: F, t3449: F, t4900: F, t4908: F, t50879: F, t50884: F, t50915: F, t50929: F, t14744: F, t1174: F, t135: F, t15359: F, t11589: F, t15293: F, t15382: F, t44525: F, t11588: F, t4928: F, t15357: F, t3448: F, t11579: F, t11584: F, t11593: F, t15313: F, t15320: F, t44517: F, t44536: F, t44540: F, t44558: F, t4919: F, t50857: F, t50861: F, t50873: F, t50964: F, t14740: F, t15338: F, t461: F, t4729: F, t15418: F, t11571: F, t14736: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51906, t51913, t51916, t51925, t51928, t51937) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2567::<F>(t1254, t3633, t1157, t1164, t14829, t3375, t14966, t3378, t15823, t225, t15800, t15808);
        let t51946 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2568::<F>(t11598, t11919, t11935, t1238, t1251, t1252, t14972, t15786, t15794, t15797, t15803, t15820, t1751, t1761, t3487, t3598, t3600, t3631, t44412, t4945, t498, t51925, t51928, t51937);
        let (t51948, t51961, t51971, t51975, t51981, t51988) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2569::<F>(t14731, t15419, t3447, t12606, t3450, t1714, t44583, t3451, t458, t44584, t4904, t44510);
        let t51993 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2570::<F>(t14753, t15402, t3447, t11509, t11566, t11576, t11580, t11585, t11594, t15376, t15395, t3449, t4900, t4908, t50879, t50884, t50915, t50929, t51948, t51961, t51971, t51975, t51981, t51988);
        let (t51995, t52013, t52019, t52022, t52036) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2571::<F>(t14744, t15402, t3447, t1174, t135, t15359, t11589, t15293, t15382, t44525, t11588, t4928);
        let t52047 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2572::<F>(t3447, t3451, t52036, t15357, t3448, t11579, t11584, t11593, t15313, t15320, t15382, t44517, t44536, t44540, t44558, t4900, t4904, t4908, t4919, t50857, t50861, t50873, t50964, t51995, t52013, t52019, t52022);
        let (t52050, t52053, t52058, t52061, t52064) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2573::<F>(t14740, t15419, t3447, t11584, t15338, t44583, t461, t4729, t15418, t1714, t11571, t14736);
    (t51906, t51913, t51916, t51946, t51993, t52047, t52050, t52053, t52058, t52061, t52064)
}
