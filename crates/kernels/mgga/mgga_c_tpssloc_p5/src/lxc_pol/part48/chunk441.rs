//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 441/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk441<F: Float>(t221: F, t2965: F, t339: F, t964: F, t995: F, t1000: F, t1020: F, t1025: F, t1046: F, t2955: F, t2960: F, t3109: F, t3114: F, t3117: F, t3123: F, t3130: F, t3134: F, t3140: F, t3143: F, t3148: F, t3153: F, t3156: F, t350: F, t973: F) -> F {
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / F::cast_from(432.0_f64);
    let t3163 = t964 * t995;
    let t3165 = -t3109 * t1025 / F::cast_from(288.0_f64) + t3114 * t1025 / F::cast_from(1536.0_f64) + t3117 * t1046 / F::cast_from(2304.0_f64) + t1020 * t3123 / F::cast_from(3072.0_f64) + t3130 * t3134 / F::cast_from(1536.0_f64) - t2960 * t1000 / F::cast_from(54.0_f64) + t3140 / F::cast_from(432.0_f64) + t973 * t3143 / F::cast_from(288.0_f64) + t973 * t3148 / F::cast_from(216.0_f64) - t973 * t3153 / F::cast_from(144.0_f64) + t3156 / F::cast_from(2304.0_f64) - t3160 + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t2955 * t350 - t3163 / F::cast_from(54.0_f64);
    t3165
}
