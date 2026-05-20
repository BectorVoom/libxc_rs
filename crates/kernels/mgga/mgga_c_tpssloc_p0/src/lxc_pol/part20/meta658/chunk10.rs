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
    let t50136 = F::new(5.0) / F::new(144.0) * t3048 * t14174 - F::new(5.0) / F::new(432.0) * t10863 * t4590 - F::new(5.0) / F::new(432.0) * t3048 * t14180 + t50110 / F::new(108.0) + t50113 / F::new(216.0) + F::new(7.0) / F::new(648.0) * t50116 - t973 * t977 * t47759 / F::new(48.0) - t973 * t977 * t47763 / F::new(48.0) - t973 * t2979 * t47684 / F::new(12.0) - t2960 * t14015 / F::new(27.0) - F::new(7.0) / F::new(81.0) * t2960 * t14018 + t50132 / F::new(288.0) - t2960 * t14198 / F::new(36.0);
    t50136
}
