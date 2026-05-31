//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2450/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2450<F: Float>(t10231: F, t13528: F, t973: F, t13532: F, t13537: F, t42972: F, t135: F, t14197: F, t10863: F, t14015: F, t14018: F, t14174: F, t14180: F, t14198: F, t2960: F, t2979: F, t3048: F, t4590: F, t47684: F, t47759: F, t47763: F, t977: F) -> F {
    let t50110 = t973 * t10231 * t13528;
    let t50113 = t973 * t10231 * t13532;
    let t50116 = t973 * t42972 * t13537;
    let t50132 = t973 * t135 * t14197;
    let t50136 = F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t3048 * t14174 - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t10863 * t4590 - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t3048 * t14180 + t50110 / F::cast_from(108.0_f64) + t50113 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t50116 - t973 * t977 * t47759 / F::cast_from(48.0_f64) - t973 * t977 * t47763 / F::cast_from(48.0_f64) - t973 * t2979 * t47684 / F::cast_from(12.0_f64) - t2960 * t14015 / F::cast_from(27.0_f64) - F::cast_from(7.0_f64) / F::cast_from(81.0_f64) * t2960 * t14018 + t50132 / F::cast_from(288.0_f64) - t2960 * t14198 / F::cast_from(36.0_f64);
    t50136
}
