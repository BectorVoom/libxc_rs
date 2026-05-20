//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta13 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk93;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk94;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk95;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk96;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk97;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk98;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk99;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta13<F: Float>(t154: F, t205: F, t131: F, t206: F, t119: F, t209: F, t191: F, t218: F, t144: F, t186: F, t189: F, t202: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t219, t220, t221) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk93::<F>(t154, t205, t131, t206, t119, t209);
        let (t222, t225) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk94::<F>(t220, t221, t191);
        let t226 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk95::<F>(t218, t225);
        let t228 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk96::<F>(t144, t186, t189, t225);
        let t229 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk97::<F>(t202);
        let t230 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk98::<F>(t229, t68);
        let t232 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk99::<F>(t228, t230);
        let (t233, t234) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk100::<F>(t232);
    (t219, t220, t221, t222, t225, t226, t228, t229, t230, t232, t233, t234)
}
