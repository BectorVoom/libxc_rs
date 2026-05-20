//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1466;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1467;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1468;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1469;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta390<F: Float>(t11661: F, t13969: F, t3506: F, t11721: F, t3493: F, t11858: F, t1226: F, t3030: F, t3481: F, t3032: F, t3505: F, t3514: F, t1174: F, t11760: F, t135: F, t11147: F, t3439: F, t11719: F, t11724: F, t11728: F, t11734: F, t11770: F, t11814: F, t1214: F, t1216: F, t1227: F, t1230: F, t1232: F, t15620: F, t248: F, t3496: F, t3508: F, t3511: F, t3515: F, t3518: F, t39097: F, t43757: F, t44668: F, t44873: F, t44879: F, t44886: F, t44890: F, t44894: F, t44896: F, t4582: F, t974: F, t11789: F, t820: F, t3577: F, t3579: F, t11737: F, t44857: F, t11791: F, t3490: F, t3252: F, t3248: F, t11665: F, t11698: F, t11683: F, t11697: F, t11673: F, t11678: F, t11679: F, t11687: F, t11877: F, t3576: F, t11668: F, t11674: F, t11692: F, t11741: F, t11774: F, t15453: F, t3243: F, t3494: F, t3516: F, t3578: F, t3580: F, t42468: F, t11647: F, t1203: F, t11859: F, t1222: F, t11797: F, t11172: F, t3521: F, t11801: F, t204: F, t486: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44904, t44906, t44918, t44927, t44929, t44932) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1466::<F>(t11661, t13969, t3506, t11721, t3493, t11858, t1226, t3030, t3481, t3032, t3505, t3514);
        let t44943 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1467::<F>(t1174, t11760, t135, t11147, t3439, t11719, t11724, t11728, t11734, t11770, t11814, t1214, t1216, t1227, t1230, t1232, t15620, t248, t3496, t3506, t3508, t3511, t3515, t3518, t39097, t43757, t44668, t44873, t44879, t44886, t44890, t44894, t44896, t44904, t44906, t44918, t44929, t44932, t4582, t974);
        let (t44953, t44965, t44968, t44972, t44976) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1468::<F>(t11789, t820, t3577, t3579, t11737, t44857, t11791, t3490, t1227, t248, t3252, t3248);
        let t44999 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1469::<F>(t11665, t11698, t11683, t11697, t3577, t11673, t11678, t11679, t11687, t11877, t3576, t11668, t11674, t11692, t11741, t11774, t1227, t15453, t3243, t3248, t3490, t3494, t3516, t3578, t3580, t42468, t44953, t44965, t44968, t44972, t44976, t4582);
        let (t45002, t45007, t45009, t45013, t45015, t45017) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1470::<F>(t11647, t1203, t11859, t1222, t11797, t3490, t11172, t1227, t248, t3521, t11801, t204, t486);
    (t44927, t44943, t44999, t45002, t45007, t45009, t45013, t45015, t45017)
}
