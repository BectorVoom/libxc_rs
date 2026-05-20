//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta89 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk506;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk507;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk508;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk509;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk510;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta89<F: Float>(t2929: F, t315: F, t323: F, t340: F, t697: F, t344: F, t221: F, t339: F, t135: F, t976: F, t271: F, t883: F, t974: F, t2770: F, t337: F, t39: F, t1887: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2930, t2931, t2932) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk506::<F>(t2929, t315, t323);
        let (t2965, t2969, t2970) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk507::<F>(t340, t697, t344, t221, t339, t135, t976);
        let t2978 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk508::<F>(t271, t883);
        let t2979 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk509::<F>(t2978, t974);
        let (t2980, t2986) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk510::<F>(t2770, t344, t337, t39, t1887);
        let t2987 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk511::<F>(t60, t976);
    (t2930, t2931, t2932, t2965, t2969, t2970, t2978, t2979, t2980, t2986, t2987)
}
