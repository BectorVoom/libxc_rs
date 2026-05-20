//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk730;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk731;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk732;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk733;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk734;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk735;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta125<F: Float>(t3158: F, t339: F, t964: F, t995: F, t1050: F, t225: F, t1053: F, t386: F, t68: F, t1057: F, t3112: F, t3032: F, t3127: F, t3031: F, t1932: F, t3131: F, t1014: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3160, t3163, t3169) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk730::<F>(t3158, t339, t964, t995, t1050, t225);
        let (t3173, t3174) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk731::<F>(t1053, t386, t68);
        let t3180 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk732::<F>(t1057, t3112);
        let (t3185, t3186) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk733::<F>(t3032, t3127, t3031);
        let t3188 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk734::<F>(t1932, t3131);
        let (t3199, t3200) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk735::<F>(t1014, t3032, t3031);
        let t3201 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk736::<F>(t1932, t360);
    (t3160, t3163, t3169, t3173, t3174, t3180, t3185, t3186, t3188, t3199, t3200, t3201)
}
