//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1590;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1591;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1592;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta417<F: Float>(t1136: F, t6037: F, t1683: F, t4819: F, t6056: F, t6053: F, t3359: F, t6052: F, t4823: F, t11352: F, t6036: F, t11137: F, t11444: F, t14702: F, t14720: F, t15194: F, t15195: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t18243: F, t14838: F, t4745: F, t11350: F, t11420: F, t18257: F, t18261: F, t18264: F, t18268: F, t3332: F, t3357: F, t436: F, t14850: F, t4786: F, t1117: F, t5989: F, t3313: F, t1671: F, t4781: F, t3264: F, t6024: F, t11190: F, t1098: F, t5983: F, t1119: F, t14845: F, t4740: F, t4782: F, t11424: F, t3259: F, t6021: F, t11136: F, t14922: F, t14923: F, t14924: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18631, t18634, t18637, t18640, t18644, t18647, t18651, t18668) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1590::<F>(t1136, t6037, t1683, t4819, t6056, t6053, t3359, t6052, t4823, t11352, t6036, t11137, t11444, t14702, t14720, t15194, t15195, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18672, t18673) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1591::<F>(t14838, t4745, t11350, t11420, t18257, t18261, t18264, t18268, t18631, t18634, t18637, t18640, t18644, t18647, t18651, t18668, t3332, t3357, t436);
        let (t18676, t18679, t18682, t18685, t18686) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1592::<F>(t14850, t4786, t1117, t5989, t3313, t1671, t4781, t3264, t6024, t11190, t1098, t5983);
        let (t18688, t18690, t18692, t18694, t18696, t18710) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1593::<F>(t1119, t18686, t14845, t1671, t4740, t4782, t11424, t5989, t3259, t6021, t11136, t11137, t14702, t14922, t14923, t14924, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
    (t18672, t18673, t18676, t18679, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18710)
}
