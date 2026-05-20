//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta144 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk813;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk814;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk815;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk816;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk817;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk818;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk819;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta144<F: Float>(t3158: F, t339: F, t964: F, t995: F, t1000: F, t1020: F, t1025: F, t1046: F, t2955: F, t2960: F, t3109: F, t3114: F, t3117: F, t3123: F, t3130: F, t3134: F, t3140: F, t3143: F, t3148: F, t3153: F, t3156: F, t350: F, t973: F, t3106: F, t349: F, t1050: F, t225: F, t1053: F, t386: F, t68: F, t1065: F, t1057: F, t3112: F, t3032: F, t3127: F, t3031: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3160, t3163, t3165) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk813::<F>(t3158, t339, t964, t995, t1000, t1020, t1025, t1046, t2955, t2960, t3109, t3114, t3117, t3123, t3130, t3134, t3140, t3143, t3148, t3153, t3156, t350, t973);
        let t3166 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk814::<F>(t3106, t3165);
        let (t3167, t3169) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk815::<F>(t3166, t349, t1050, t225);
        let (t3173, t3174) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk816::<F>(t1053, t386, t68);
        let t3175 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk817::<F>(t1065);
        let t3176 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk818::<F>(t3174, t3175);
        let t3180 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk819::<F>(t1057, t3112);
        let (t3185, t3186) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk820::<F>(t3032, t3127, t3031);
    (t3160, t3163, t3166, t3167, t3169, t3173, t3174, t3175, t3176, t3180, t3185, t3186)
}
