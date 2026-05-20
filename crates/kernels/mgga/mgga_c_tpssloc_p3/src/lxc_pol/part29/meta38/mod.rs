//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta38 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk268;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk269;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk270;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk271;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta38<F: Float>(t730: F, t731: F, t177: F, t172: F, t688: F, t690: F, t694: F, t699: F, t180: F, t118: F, t168: F, t181: F, t677: F, t680: F, t705: F, t725: F, t157: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t732, t738, t739) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk268::<F>(t730, t731, t177);
        let (t740, t745) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk269::<F>(t172, t739, t688, t690, t694, t699);
        let t746 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk270::<F>(t180);
        let (t747, t750) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk271::<F>(t745, t746, t118, t168, t181, t677, t680, t705, t725, t732, t740);
        let t751 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk272::<F>(t157, t750);
    (t732, t738, t739, t740, t745, t746, t747, t750, t751)
}
