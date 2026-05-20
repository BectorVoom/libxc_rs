//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta416 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1585;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1586;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1587;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1588;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta416<F: Float>(t4889: F, t4916: F, t1653: F, t7319: F, t4919: F, t15293: F, t4928: F, t8034: F, t4934: F, t1184: F, t460: F, t6144: F, t1178: F, t16558: F, t1177: F, t6138: F, t11556: F, t1174: F, t1187: F, t15401: F, t15405: F, t15422: F, t18321: F, t3447: F, t4913: F, t4931: F, t18442: F, t18473: F, t18535: F, t225: F, t68: F, t484: F, t18215: F, t3440: F, t18211: F, t5012: F, t3578: F, t17691: F, t4972: F, t4582: F, t15615: F, t17686: F, t1155: F, t6069: F, t1695: F, t4857: F, t6088: F, t6085: F, t3403: F, t6084: F, t4861: F, t11285: F, t6068: F, t11310: F, t11365: F, t15126: F, t15136: F, t15146: F, t15207: F, t18247: F, t3376: F, t3401: F, t4802: F, t4824: F, t4840: F, t4862: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18536, t18543, t18546, t18550, t18554) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1585::<F>(t4889, t4916, t1653, t7319, t4919, t15293, t4928, t8034, t4934, t1184, t460, t6144);
        let t18569 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1586::<F>(t18554, t4934, t1178, t16558, t1177, t1184, t460, t6138, t11556, t1174, t1187, t15401, t15405, t15422, t18321, t18536, t18543, t18546, t18550, t3447, t4889, t4913, t4931);
        let (t18571, t18572, t18574, t18577, t18580, t18583) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1587::<F>(t18442, t18473, t18535, t18569, t225, t68, t484, t18215, t3440, t18211, t1653, t5012);
        let (t18584, t18590, t18594, t18603, t18606, t18609) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1588::<F>(t18583, t3578, t17691, t4972, t4582, t15615, t17686, t1155, t6069, t1695, t4857, t6088);
        let t18630 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1589::<F>(t1155, t6085, t3403, t6084, t4857, t4861, t11285, t6068, t11310, t11365, t15126, t15136, t15146, t15207, t18247, t18603, t18606, t18609, t3376, t3401, t4802, t4824, t4840, t4862);
    (t18571, t18572, t18574, t18577, t18580, t18584, t18590, t18594, t18630)
}
