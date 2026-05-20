//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta141 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk788;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk789;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk790;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk791;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta141<F: Float>(t1009: F, t990: F, t1011: F, t1019: F, t1004: F, t1040: F, t2786: F, t2789: F, t2796: F, t2839: F, t2847: F, t2937: F, t2939: F, t2942: F, t2946: F, t2950: F, t2954: F, t360: F, t1021: F, t248: F, t1013: F, t361: F, t363: F, t3037: F, t3033: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3112, t3113, t3114) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk788::<F>(t1009, t990, t1011, t1019);
        let t3117 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk789::<F>(t1004, t1040);
        let t3120 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk790::<F>(t2786, t2789, t2796, t2839, t2847, t2937, t2939, t2942, t2946, t2950, t2954);
        let (t3121, t3123, t3127, t3128, t3129, t3130) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk791::<F>(t3120, t360, t1021, t248, t1013, t361, t363, t3037, t3033);
        let t3131 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk792::<F>(t360);
    (t3112, t3113, t3114, t3117, t3120, t3121, t3123, t3127, t3128, t3129, t3130, t3131)
}
