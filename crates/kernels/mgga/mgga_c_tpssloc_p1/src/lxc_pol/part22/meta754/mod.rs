//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2533;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2534;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2535;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta754<F: Float>(t449: F, t71289: F, t71308: F, t1671: F, t63750: F, t18686: F, t4782: F, t14845: F, t6021: F, t18835: F, t4740: F, t11310: F, t11350: F, t1136: F, t1155: F, t15171: F, t15225: F, t18612: F, t18616: F, t18640: F, t18786: F, t4835: F, t51382: F, t51389: F, t51727: F, t6052: F, t6084: F, t71095: F, t71097: F, t71217: F, t71245: F, t136: F, t3297: F, t71138: F, t21746: F, t699: F, t21750: F, t50827: F, t50834: F, t63291: F, t63306: F, t63308: F, t63841: F, t63843: F, t63845: F, t51058: F, t63332: F, t63334: F, t63336: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71144: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F, t43816: F, t43820: F, t51073: F, t51082: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71191: F, t71195: F, t71199: F, t71203: F, t71206: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t71310, t71313, t71315, t71317, t71319, t71322) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2533::<F>(t449, t71289, t71308, t1671, t63750, t18686, t4782, t14845, t6021, t18835, t4740, t11310, t11350, t1136, t1155, t15171, t15225, t18612, t18616, t18640, t18786, t4835, t51382, t51389, t51727, t6052, t6084, t71095, t71097, t71217, t71245);
        let (t71333, t71335, t71337, t71343) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2534::<F>(t136, t3297, t71138, t21746, t699, t21750, t50827, t50834, t63291, t63306, t63308, t63841, t63843, t63845);
        let t71371 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2535::<F>(t50834, t51058, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let t71389 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2536::<F>(t43816, t43820, t51073, t51082, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
    (t71310, t71313, t71315, t71317, t71319, t71322, t71333, t71335, t71337, t71343, t71371, t71389)
}
