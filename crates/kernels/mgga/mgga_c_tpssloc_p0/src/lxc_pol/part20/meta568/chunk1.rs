//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2129/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2129<F: Float>(t1012: F, t1015: F, t1017: F, t10444: F, t10632: F, t2924: F, t10510: F, t3114: F, t10454: F, t3117: F, t10891: F, t10895: F) -> (F, F, F, F, F) {
    let t42658 = t1012 * t1015 * t10444 * t1017;
    let t42671 = t10632 * t2924;
    let t42721 = t3114 * t10510;
    let t42729 = t3117 * t10454;
    let t42731 = t10891 * t10895;
    (t42658, t42671, t42721, t42729, t42731)
}
