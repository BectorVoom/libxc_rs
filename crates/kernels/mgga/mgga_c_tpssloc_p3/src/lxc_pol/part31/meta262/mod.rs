//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta262 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1095;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1096;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1097;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1098;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta262<F: Float>(t6966: F, t6974: F, t1338: F, t2085: F, t1352: F, t553: F, t7191: F, t1332: F, t1336: F, t2089: F, t544: F, t6971: F, t6980: F, t6984: F, t1378: F, t1375: F, t1386: F, t2092: F, t3758: F, t3882: F, t568: F, t6893: F, t6904: F, t6909: F, t7174: F, t7176: F, t7179: F, t7192: F, t7194: F, t7199: F, t533: F, t1390: F, t2095: F, t6999: F, t113: F, t1266: F, t1393: F, t1983: F, t2036: F, t2040: F, t2075: F, t2079: F, t2096: F, t2314: F, t4034: F, t510: F, t574: F, t650: F, t652: F, t672: F, t6876: F, t7040: F, t7042: F, t7050: F, t7057: F, t7061: F, t7156: F, t7166: F, t7171: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7202, t7204, t7208) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1095::<F>(t6966, t6974, t1338, t2085);
        let (t7209, t7211, t7213) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1096::<F>(t1352, t7208, t553, t7191, t1332, t1336, t2089, t544, t6971, t6980, t6984, t7202, t7204);
        let t7214 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1097::<F>(t1378, t7213);
        let t7216 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1098::<F>(t1375, t1386, t2092, t3758, t3882, t568, t6893, t6904, t6909, t7174, t7176, t7179, t7192, t7194, t7199, t7214);
        let (t7217, t7218, t7220, t7222) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1099::<F>(t533, t7216, t1390, t2095, t6999, t113, t1266, t1393, t1983, t2036, t2040, t2075, t2079, t2096, t2314, t4034, t510, t574, t650, t652, t672, t6876, t7040, t7042, t7050, t7057, t7061, t7156, t7166, t7171);
    (t7202, t7204, t7208, t7209, t7211, t7213, t7214, t7216, t7217, t7218, t7220, t7222)
}
