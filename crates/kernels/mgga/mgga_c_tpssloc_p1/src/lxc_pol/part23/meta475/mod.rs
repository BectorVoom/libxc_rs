//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta475 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1420;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1421;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1422;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1423;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1424;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1425;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1426;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta475<F: Float>(t78000: F, t78019: F, t78082: F, t78112: F, t1147: F, t1156: F, t1164: F, t18915: F, t6098: F, t22222: F, t4869: F, t6085: F, t6105: F, t4861: F, t72062: F, t5988: F, t11277: F, t43969: F, t50834: F, t71335: F, t71337: F, t77959: F, t77963: F, t77967: F, t77971: F, t77975: F, t77979: F, t77983: F, t77989: F, t77992: F, t77995: F, t77998: F, t63332: F, t63334: F, t63888: F, t63893: F, t63911: F, t71142: F, t71144: F, t71146: F, t71152: F, t71154: F, t71156: F, t71408: F, t78002: F, t78005: F, t44027: F, t50846: F, t71470: F, t71472: F, t71474: F, t78026: F, t78029: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F, t78078: F, t78080: F, t44053: F, t63361: F, t78057: F, t78084: F, t78087: F, t78090: F, t78093: F, t78095: F, t78097: F, t78100: F, t78103: F, t78105: F, t78107: F, t78109: F, t1099: F, t1118: F, t44075: F, t44077: F, t43942: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t78114, t78118, t78120, t78122, t78125) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1420::<F>(t78000, t78019, t78082, t78112, t1147, t1156, t1164, t18915, t6098, t22222, t4869, t6085, t6105);
        let (t78128, t78129, t78132, t78147) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1421::<F>(t1164, t4861, t72062, t5988, t11277, t43969, t50834, t71335, t71337, t77959, t77963, t77967, t77971, t77975, t77979, t77983, t77989, t77992, t77995, t77998);
        let t78162 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1422::<F>(t63332, t63334, t63888, t63893, t63911, t71142, t71144, t71146, t71152, t71154, t71156, t71408, t78002, t78005);
        let t78177 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1423::<F>(t44027, t50846, t71470, t71472, t71474, t78026, t78029, t78033, t78037, t78041, t78045, t78049, t78078, t78080);
        let t78191 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1424::<F>(t44053, t63361, t78057, t78084, t78087, t78090, t78093, t78095, t78097, t78100, t78103, t78105, t78107, t78109);
        let (t78196, t78199, t78211) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1425::<F>(t1099, t1118, t78147, t78162, t78177, t78191, t44075, t44077, t78129, t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057);
        let t78223 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1426::<F>(t43942, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
    (t78114, t78118, t78120, t78122, t78125, t78128, t78129, t78132, t78196, t78199, t78211, t78223)
}
