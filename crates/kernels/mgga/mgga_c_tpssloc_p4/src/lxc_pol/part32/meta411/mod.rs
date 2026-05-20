//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1583;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1584;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1585;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1586;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1587;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta411<F: Float>(t248: F, t3521: F, t5975: F, t1227: F, t1409: F, t15701: F, t15700: F, t3578: F, t1735: F, t4729: F, t18232: F, t4900: F, t3450: F, t5398: F, t3449: F, t18237: F, t4908: F, t3448: F, t6138: F, t3451: F, t6144: F, t18225: F, t11583: F, t5392: F, t18221: F, t15320: F, t4904: F, t15313: F, t4919: F, t11531: F, t15265: F, t15376: F, t3447: F, t4901: F, t15395: F, t18206: F, t15338: F, t3431: F, t6126: F, t1174: F, t6130: F, t11539: F, t6119: F, t4889: F, t4896: F, t18215: F, t11570: F, t11569: F, t1180: F, t15284: F, t15287: F, t15300: F, t15307: F, t18321: F, t4937: F, t18211: F, t15382: F, t15390: F, t1171: F, t6109: F, t6011: F, t699: F, t11219: F, t136: F, t3297: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18392, t18393, t18397, t18401, t18404) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1583::<F>(t248, t3521, t5975, t1227, t1409, t15701, t15700, t3578, t1735, t4729, t18232, t4900);
        let (t18410, t18413, t18417, t18421, t18424, t18427) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1584::<F>(t3450, t5398, t3449, t18237, t4908, t3448, t6138, t3451, t6144, t18225, t11583, t5392);
        let t18442 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1585::<F>(t18427, t3449, t18221, t4908, t15320, t4904, t15313, t4919, t11531, t15265, t15376, t18404, t18410, t18413, t18417, t18421, t18424, t3447, t4901);
        let (t18443, t18447, t18452, t18455, t18458, t18460) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1586::<F>(t15395, t18206, t15338, t4904, t3447, t3431, t6126, t1174, t6130, t11539, t6119, t4889, t4896);
        let t18473 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1587::<F>(t18215, t4900, t11570, t5392, t11569, t1180, t15284, t15287, t15300, t15307, t18321, t18443, t18447, t18452, t18455, t18458, t18460, t3447, t4889, t4937);
        let (t18475, t18484, t18489, t18494, t18497, t18499) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1588::<F>(t18211, t4900, t15382, t15390, t1171, t6109, t6011, t699, t11219, t18206, t136, t3297);
    (t18392, t18393, t18397, t18401, t18442, t18473, t18475, t18484, t18489, t18494, t18497, t18499)
}
