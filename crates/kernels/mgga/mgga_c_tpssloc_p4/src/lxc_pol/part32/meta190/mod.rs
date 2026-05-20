//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta190 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk928;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk929;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk930;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk931;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk932;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta190<F: Float>(t1755: F, t5079: F, t1215: F, t1751: F, t1246: F, t493: F, t5052: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1756: F, t1758: F, t3604: F, t3610: F, t3624: F, t470: F, t494: F, t4964: F, t5064: F, t5069: F, t5073: F, t5076: F, t1241: F, t1238: F, t1252: F, t1761: F, t3487: F, t3593: F, t4941: F, t4943: F, t4945: F, t4947: F, t498: F, t5053: F, t5055: F, t5060: F, t1763: F, t3640: F, t1254: F, t1256: F, t193: F, t336: F, t4700: F, t4739: F, t4742: F, t4744: F, t4747: F, t4784: F, t4788: F, t4866: F, t4868: F, t4871: F, t4873: F, t4877: F, t4881: F, t4886: F, t28: F, t265: F, t504: F, t4324: F, t1081: F, t1260: F, t1409: F, t1534: F, t1649: F, t1768: F, t3966: F, t4332: F, t506: F, t52: F, t607: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F, t4712: F, t671: F, t88: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5080, t5083, t5084, t5086, t5088) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk928::<F>(t1755, t5079, t1215, t1751, t1246, t493, t5052, t1201, t1244, t1247, t1249, t1729, t1756, t1758, t3604, t3610, t3624, t470, t494, t4964, t5064, t5069, t5073, t5076);
        let (t5089, t5091) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk929::<F>(t1241, t5088, t1238, t1252, t1761, t3487, t3593, t4941, t4943, t4945, t4947, t498, t5053, t5055, t5060);
        let (t5095, t5098) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk930::<F>(t1763, t3640, t1254, t1256, t193, t336, t4700, t4739, t4742, t4744, t4747, t4784, t4788, t4866, t4868, t4871, t4873, t4877, t4881, t4886, t5091);
        let (t5099, t5106) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk931::<F>(t28, t265, t504, t4324, t5098, t1081, t1260, t1409, t1534, t1649, t1768, t3966, t4332, t506, t52, t607, t873, dens_threshold, rho1, zeta_threshold);
        let t5107 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk932::<F>(t4712, t5106);
        let t5113 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk933::<F>(t671, t88);
    (t5080, t5083, t5084, t5086, t5088, t5089, t5091, t5095, t5099, t5107, t5113)
}
