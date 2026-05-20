//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2128;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta568<F: Float>(t3147: F, t698: F, t973: F, t10981: F, t2960: F, t10984: F, t1004: F, t10956: F, t10863: F, t3053: F, t10516: F, t3113: F, t1012: F, t1015: F, t1017: F, t10444: F, t10632: F, t2924: F, t10510: F, t3114: F, t10454: F, t3117: F, t10891: F, t10895: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42613, t42619, t42622, t42648, t42651, t42653) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2128::<F>(t3147, t698, t973, t10981, t2960, t10984, t1004, t10956, t10863, t3053, t10516, t3113);
        let (t42658, t42671, t42721, t42729, t42731) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2129::<F>(t1012, t1015, t1017, t10444, t10632, t2924, t10510, t3114, t10454, t3117, t10891, t10895);
    (t42613, t42619, t42622, t42648, t42651, t42653, t42658, t42671, t42721, t42729, t42731)
}
