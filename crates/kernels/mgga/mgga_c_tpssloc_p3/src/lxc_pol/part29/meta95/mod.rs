//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta95 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk621;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk622;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk623;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk624;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk625;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta95<F: Float>(t2122: F, t497: F, t462: F, t131: F, t2120: F, t467: F, t480: F, t1932: F, t3: F, t52: F, t225: F, t461: F, t479: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2123 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk621::<F>(t2122, t497);
        let (t2124, t2127) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk622::<F>(t2123, t462, t131, t2120);
        let (t2128, t2130, t2131, t2132) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk623::<F>(t2127, t467, t480, t1932);
        let t2133 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk624::<F>(t3, t52);
        let t2134 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk625::<F>(t2132, t2133);
        let (t2135, t2136) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk626::<F>(t225, t461, t479);
    (t2123, t2124, t2127, t2128, t2130, t2131, t2132, t2133, t2134, t2135, t2136)
}
