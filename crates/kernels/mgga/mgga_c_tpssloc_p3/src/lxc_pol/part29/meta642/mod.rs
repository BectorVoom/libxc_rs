//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta642 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2113;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2114;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2115;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2116;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta642<F: Float>(t87233: F, t25068: F, t2703: F, t81764: F, t23127: F, t4257: F, t1512: F, t81807: F, t25146: F, t2686: F, t81824: F, t81821: F, t23053: F, t4236: F, t13173: F, t6614: F, t23041: F, t13186: F, t6621: F, t81770: F, t81772: F, t81785: F, t87222: F, t87224: F, t87226: F, t23040: F, t4166: F, t831: F, t81808: F, t4191: F, t81749: F, t4240: F, t13248: F, t25084: F, t13326: F, t23146: F, t13210: F, t13306: F, t13231: F, t81789: F, t81795: F, t81797: F, t81799: F, t81810: F, t81825: F, t81836: F, t81850: F, t81853: F, t13353: F, t13225: F, t23069: F, t4159: F, t23062: F, t25106: F, t13176: F, t6613: F, t2681: F) -> (F, F, F, F, F, F, F, F) {
        let (t87234, t87235, t87237, t87241, t87243, t87245, t87248, t87249) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2113::<F>(t87233, t25068, t2703, t81764, t23127, t4257, t1512, t81807, t25146, t2686, t81824, t81821);
        let t87259 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2114::<F>(t23053, t4236, t13173, t6614, t23041, t13186, t6621, t81770, t81772, t81785, t87222, t87224, t87226, t87234, t87235, t87237, t87241, t87243, t87245, t87248, t87249);
        let (t87263, t87268, t87271, t87273, t87274, t87276, t87278) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2115::<F>(t23040, t4166, t831, t81808, t4191, t81749, t4240, t13248, t25084, t13326, t23146, t13210);
        let t87286 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2116::<F>(t13306, t23146, t13231, t25084, t81789, t81795, t81797, t81799, t81810, t81825, t81836, t81850, t81853, t87263, t87268, t87271, t87273, t87274, t87276, t87278);
        let (t87287, t87289, t87292, t87293, t87296, t87298) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2117::<F>(t13353, t23146, t13225, t23069, t4159, t23062, t25106, t13176, t6613, t831, t25146, t2681);
    (t87259, t87286, t87287, t87289, t87292, t87293, t87296, t87298)
}
