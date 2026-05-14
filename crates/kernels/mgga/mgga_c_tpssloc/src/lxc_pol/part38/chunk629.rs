//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 629/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk629<F: Float>(t221: F, t2965: F, t339: F, t964: F, t995: F, t1000: F, t1020: F, t1025: F, t1046: F, t2955: F, t2960: F, t3109: F, t3114: F, t3117: F, t3123: F, t3130: F, t3134: F, t3140: F, t3143: F, t3148: F, t3153: F, t3156: F, t350: F, t973: F) -> (F, F, F, F) {
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / 432.0;
    let t3163 = t964 * t995;
    let t3165 = -t3109 * t1025 / 288.0 + t3114 * t1025 / 1536.0 + t3117 * t1046 / 2304.0 + t1020 * t3123 / 3072.0 + t3130 * t3134 / 1536.0 - t2960 * t1000 / 54.0 + t3140 / 432.0 + t973 * t3143 / 288.0 + t973 * t3148 / 216.0 - t973 * t3153 / 144.0 + t3156 / 2304.0 - t3160 + 11.0 / 108.0 * t2955 * t350 - t3163 / 54.0;
    (t3158, t3160, t3163, t3165)
}
