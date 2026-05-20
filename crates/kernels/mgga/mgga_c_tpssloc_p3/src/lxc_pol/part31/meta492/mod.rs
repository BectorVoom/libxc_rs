//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1678;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1679;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta492<F: Float>(t26429: F, t1338: F, t7918: F, t1352: F, t5287: F, t7208: F, t27051: F, t553: F, t1332: F, t1336: F, t1814: F, t2089: F, t22728: F, t22731: F, t22746: F, t22753: F, t22896: F, t24108: F, t24110: F, t26434: F, t26437: F, t26449: F, t26463: F, t26468: F, t5230: F, t544: F, t7211: F, t7934: F, t27095: F, t1378: F, t1375: F, t1386: F, t16022: F, t16439: F, t1843: F, t2092: F, t22676: F, t24095: F, t26475: F, t27067: F, t27068: F, t27070: F, t3758: F, t3882: F, t5215: F, t5321: F, t568: F, t7199: F, t7214: F, t7937: F, t1842: F, t7213: F, t3887: F, t1807: F, t7191: F, t16460: F, t22908: F, t22910: F, t22922: F, t22928: F, t22941: F, t24082: F, t24156: F, t24157: F, t5354: F, t7194: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27097, t27098, t27103, t27105, t27113) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1678::<F>(t26429, t1338, t7918, t1352, t5287, t7208, t27051, t553, t1332, t1336, t1814, t2089, t22728, t22731, t22746, t22753, t22896, t24108, t24110, t26434, t26437, t26449, t26463, t26468, t5230, t544, t7211, t7934);
        let (t27114, t27115, t27127) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1679::<F>(t27095, t27113, t1378, t1375, t1386, t16022, t16439, t1843, t2092, t22676, t24095, t26475, t27067, t27068, t27070, t3758, t3882, t5215, t5321, t568, t7199, t7214, t7937);
        let (t27132, t27137, t27141) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1680::<F>(t1842, t7213, t3887, t1807, t7191, t1375, t16460, t1843, t2092, t22908, t22910, t22922, t22928, t22941, t24082, t24156, t24157, t5215, t5321, t5354, t568, t7194, t7199, t7214);
    (t27097, t27098, t27103, t27105, t27114, t27115, t27127, t27132, t27137, t27141)
}
