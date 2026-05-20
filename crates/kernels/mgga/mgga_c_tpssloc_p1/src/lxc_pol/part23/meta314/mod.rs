//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1069;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1070;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1071;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta314<F: Float>(t21723: F, t3315: F, t11190: F, t11444: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t11459: F, t423: F, t11310: F, t11365: F, t1148: F, t15126: F, t15136: F, t15207: F, t21827: F, t21901: F, t21907: F, t21939: F, t21942: F, t21947: F, t21952: F, t21956: F, t21958: F, t21960: F, t3357: F, t3401: F, t436: F, t4835: F, t6037: F, t6069: F, t6085: F, t6088: F, t21898: F, t300: F, t1763: F, t6274: F, t11947: F, t193: F, t21726: F, t21728: F, t21730: F, t21732: F, t21812: F, t21815: F, t21829: F, t21832: F, t21835: F, t21897: F, t336: F) -> (F, F, F, F, F, F, F) {
        let (t21961, t21963, t21975) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1069::<F>(t21723, t3315, t11190, t11444, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778);
        let (t21988, t21990) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1070::<F>(t11459, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778, t423);
        let t21991 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1071::<F>(t11310, t11365, t1148, t15126, t15136, t15207, t21827, t21901, t21907, t21939, t21942, t21947, t21952, t21956, t21958, t21960, t21963, t21975, t21990, t3357, t3401, t436, t4835, t6037, t6069, t6085, t6088);
        let (t21993, t21999) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1072::<F>(t21898, t21991, t300, t1763, t6274, t11947, t193, t21726, t21728, t21730, t21732, t21812, t21815, t21829, t21832, t21835, t21897, t21901, t336);
    (t21961, t21963, t21975, t21988, t21990, t21993, t21999)
}
