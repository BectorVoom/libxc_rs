//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta106 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk720;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk721;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk722;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk723;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta106<F: Float>(t323: F, t300: F, t938: F, t964: F, t969: F, t615: F, t972: F, t340: F, t697: F, t344: F, t221: F, t339: F, t135: F, t976: F, t979: F, t973: F, t986: F, t271: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2931, t2932) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk720::<F>(t323);
        let t2940 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk721::<F>(t300, t938);
        let (t2958, t2960) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk722::<F>(t964, t969, t615, t972);
        let (t2965, t2967, t2969, t2970) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk723::<F>(t340, t697, t344, t221, t339, t135, t976);
        let (t2972, t2975, t2978) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk724::<F>(t2970, t979, t973, t135, t986, t271, t883);
    (t2931, t2932, t2940, t2958, t2960, t2965, t2967, t2969, t2970, t2972, t2975, t2978)
}
