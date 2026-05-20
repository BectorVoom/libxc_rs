//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2051/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2051<F: Float>(t2109: F, t83728: F, t83737: F, t24525: F, t9239: F, t39063: F, t7245: F, t2108: F, t2240: F, t2244: F, t39049: F, t9231: F) -> (F, F, F, F, F, F, F) {
    let t85473 = t2109 * t83728;
    let t85476 = t2109 * t83737;
    let t85480 = t9239 * t24525;
    let t85501 = t39063 * t7245;
    let t85507 = t2240 * t2244 * t2108;
    let t85510 = t39049 * t7245;
    let t85514 = t9231 * t24525;
    (t85473, t85476, t85480, t85501, t85507, t85510, t85514)
}
