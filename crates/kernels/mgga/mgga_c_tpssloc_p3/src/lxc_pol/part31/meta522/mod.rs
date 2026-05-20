//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1734;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1735;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta522<F: Float>(t6420: F, t7208: F, t6415: F, t1825: F, t27097: F, t1336: F, t24108: F, t24110: F, t26427: F, t26429: F, t26437: F, t28161: F, t28165: F, t28169: F, t28183: F, t5234: F, t7932: F, t29342: F, t1378: F, t2091: F, t3887: F, t6460: F, t1375: F, t20029: F, t20044: F, t20060: F, t2092: F, t24156: F, t24157: F, t26361: F, t26475: F, t28207: F, t28211: F, t28214: F, t28234: F, t5215: F, t5321: F, t6440: F, t6461: F, t7194: F, t7925: F, t7937: F, t29314: F, t533: F, t1390: F, t26905: F, t7687: F, t19451: F, t1983: F, t2036: F, t2040: F, t2079: F, t22574: F, t28002: F, t28030: F, t29211: F, t29214: F, t29219: F, t29222: F, t29241: F, t29243: F, t29247: F, t29252: F, t4028: F, t574: F, t6287: F, t6468: F, t652: F, t7458: F, t7685: F, t7796: F, t7802: F, t7904: F, t7943: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t29343, t29345, t29349, t29359) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1734::<F>(t6420, t7208, t6415, t1825, t27097, t1336, t24108, t24110, t26427, t26429, t26437, t28161, t28165, t28169, t28183, t5234, t7932);
        let (t29360, t29361, t29372, t29375) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1735::<F>(t29342, t29359, t1378, t2091, t3887, t6460, t1375, t20029, t20044, t20060, t2092, t24156, t24157, t26361, t26475, t28207, t28211, t28214, t28234, t5215, t5321, t6440, t6461, t7194, t7925, t7937);
        let (t29376, t29377, t29378, t29380, t29394) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1736::<F>(t29314, t29375, t533, t1390, t26905, t7687, t19451, t1983, t2036, t2040, t2079, t22574, t28002, t28030, t29211, t29214, t29219, t29222, t29241, t29243, t29247, t29252, t4028, t574, t6287, t6468, t652, t7458, t7685, t7796, t7802, t7904, t7943);
    (t29343, t29345, t29349, t29360, t29361, t29372, t29376, t29377, t29378, t29380, t29394)
}
