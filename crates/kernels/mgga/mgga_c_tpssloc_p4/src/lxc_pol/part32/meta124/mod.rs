//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk721;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk722;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta124<F: Float>(t1023: F, t248: F, t3101: F, t1020: F, t1017: F, t1030: F, t1015: F, t1012: F, t1009: F, t990: F, t1011: F, t1019: F, t1004: F, t1040: F, t1013: F, t361: F, t363: F, t3037: F, t3033: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3103, t3104, t3108, t3109, t3112, t3114) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk721::<F>(t1023, t248, t3101, t1020, t1017, t1030, t1015, t1012, t1009, t990, t1011, t1019);
        let (t3117, t3127, t3128, t3129, t3130) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk722::<F>(t1004, t1040, t1013, t361, t363, t3037, t3033);
        let t3131 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk723::<F>(t360);
    (t3103, t3104, t3108, t3109, t3112, t3114, t3117, t3127, t3128, t3129, t3130, t3131)
}
