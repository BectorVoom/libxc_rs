//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta202 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk903;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk904;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk905;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk906;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta202<F: Float>(t25: F, t28: F, t5397: F, zeta_threshold: F, t31: F, t65: F, t1410: F, t1426: F, t2267: F, t5392: F, t43: F, t48: F, t480: F, t2274: F, t55: F, sigma2: F, t1420: F, t1423: F, t2282: F, t39: F, t51: F, t56: F, t33: F, t2291: F, t634: F, t2298: F, t638: F, t72: F, t1411: F, t1427: F, t1434: F, t5393: F, t66: F, t80: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5398 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk903::<F>(t25, t28, t5397, zeta_threshold);
        let t5399 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk904::<F>(t31, t5398);
        let (t5400, t5403, t5408, t5411, t5416, t5421, t5424) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk905::<F>(t5399, t65, t1410, t1426, t2267, t5392, t43, t5398, t48, t480, t2274, t55, sigma2);
        let (t5427, t5428) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk906::<F>(t1420, t1423, t2282, t39, t51, t5408, t5411, t5416, t5421, t5424, t56, t33);
        let (t5441, t5442, t5445) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk907::<F>(t2291, t5392, t5398, t634, t2298, t638, t72, t1411, t1427, t1434, t5393, t5400, t5403, t5428, t66, t80);
    (t5398, t5399, t5400, t5403, t5408, t5411, t5416, t5427, t5428, t5441, t5442, t5445)
}
