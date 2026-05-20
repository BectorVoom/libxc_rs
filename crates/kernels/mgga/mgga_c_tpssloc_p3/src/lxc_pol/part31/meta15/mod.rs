//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta15 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk110;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk111;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk112;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk113;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk114;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk115;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk116;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk117;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta15<F: Float>(t242: F, t244: F, t248: F, t219: F, t222: F, t238: F, t218: F, t225: F, t68: F, t235: F, t226: F, t144: F, t186: F, t189: F, t193: F, t202: F, t118: F, t120: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t249 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk110::<F>(t242, t244, t248);
        let t252 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk111::<F>(t219, t222, t238, t249);
        let (t253, t254) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk112::<F>(t218, t252, t225, t68);
        let t255 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk113::<F>(t235, t252);
        let (t257, t258) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk114::<F>(t226, t255);
        let t259 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk115::<F>(t254, t258);
        let (t261, t262) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk116::<F>(t253, t259);
        let t265 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk117::<F>(t144, t186, t189, t193, t202, t262);
        let t268 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk118::<F>(t118, t120);
    (t249, t252, t253, t254, t255, t257, t258, t259, t261, t262, t265, t268)
}
