//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1230;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1231;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1232;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta344<F: Float>(t41274: F, t185: F, t39110: F, t707: F, t2447: F, t32: F, t2659: F, t9929: F, t9932: F, t31: F, t717: F, t9898: F, t2658: F, t39103: F, t607: F, t9862: F, t2250: F, t4194: F, t750: F, t39658: F, t41266: F, t41270: F, t41273: F, t6589: F, t68: F, t13151: F, t1891: F, t225: F, t228: F, t230: F, t2379: F, t2553: F, t2667: F, t2671: F, t2672: F, t2675: F, t40848: F, t40972: F, t40977: F, t41241: F, t41242: F, t41244: F, t41245: F, t41248: F, t41249: F, t41263: F, t4225: F, t822: F, t824: F, t825: F, t9516: F, t9938: F, t9947: F, t9950: F, t9951: F, t9954: F, t232: F, t2617: F, t9670: F, t831: F, t13254: F, t237: F, t249: F, t2618: F, t2623: F, t2645: F, t41123: F, t41130: F, t41132: F, t41134: F, t41139: F, t41231: F, t41237: F, t4178: F, t817: F, t819: F, t820: F, t9618: F, t9626: F, t9634: F, t9663: F, t9960: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41275, t41278, t41281, t41283, t41286) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1230::<F>(t41274, t185, t39110, t707, t2447, t32, t2659, t9929, t9932, t31, t717, t9898);
        let (t41289, t41292, t41296, t41297) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1231::<F>(t185, t2658, t39103, t607, t707, t9862, t2250, t4194, t750, t39658, t41266, t41270, t41273, t41275, t41278, t41281, t41283, t41286);
        let t41332 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1232::<F>(t6589, t68, t13151, t1891, t225, t228, t230, t2379, t2553, t2667, t2671, t2672, t2675, t40848, t40972, t40977, t41241, t41242, t41244, t41245, t41248, t41249, t41263, t41297, t4225, t822, t824, t825, t9516, t9938, t9947, t9950, t9951, t9954);
        let (t41333, t41343) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1233::<F>(t232, t41332, t2617, t9670, t831, t13254, t237, t249, t2618, t2623, t2645, t41123, t41130, t41132, t41134, t41139, t41231, t41237, t4178, t817, t819, t820, t9618, t9626, t9634, t9663, t9960);
    (t41275, t41278, t41281, t41283, t41286, t41289, t41292, t41296, t41333, t41343)
}
