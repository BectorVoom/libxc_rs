//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta190 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk854;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk855;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk856;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk857;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta190<F: Float>(t1763: F, t3640: F, t1254: F, t1256: F, t193: F, t336: F, t4700: F, t4739: F, t4742: F, t4744: F, t4747: F, t4784: F, t4788: F, t4866: F, t4868: F, t4871: F, t4873: F, t4877: F, t4881: F, t4886: F, t5091: F, t28: F, t265: F, t504: F, t4324: F, t1081: F, t1260: F, t1409: F, t1534: F, t1649: F, t1768: F, t3966: F, t4332: F, t506: F, t52: F, t607: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F, t4712: F, t671: F, t88: F, t1268: F, t1458: F, t2314: F, t4026: F, t4028: F, t4072: F, t1390: F, t1845: F, t531: F) -> (F, F, F, F, F, F, F) {
        let (t5095, t5098) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk854::<F>(t1763, t3640, t1254, t1256, t193, t336, t4700, t4739, t4742, t4744, t4747, t4784, t4788, t4866, t4868, t4871, t4873, t4877, t4881, t4886, t5091);
        let (t5099, t5106) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk855::<F>(t28, t265, t504, t4324, t5098, t1081, t1260, t1409, t1534, t1649, t1768, t3966, t4332, t506, t52, t607, t873, dens_threshold, rho1, zeta_threshold);
        let t5107 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk856::<F>(t4712, t5106);
        let t5113 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk857::<F>(t671, t88);
        let (t5118, t5122, t5126) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk858::<F>(t1268, t1458, t2314, t4026, t4028, t4072, t5113, t671, t1390, t1845, t193, t531);
    (t5095, t5099, t5107, t5113, t5118, t5122, t5126)
}
