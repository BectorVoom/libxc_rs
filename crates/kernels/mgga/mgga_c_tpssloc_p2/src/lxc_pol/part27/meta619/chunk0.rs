//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2097/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2097<F: Float>(t6695: F, t82632: F, t23317: F, t23384: F, t225: F, t23572: F, t23587: F, t6698: F, t3166: F, t6688: F, t23399: F, t6692: F, t82573: F) -> (F, F, F, F, F, F, F) {
    let t83368 = t82632 * t6695;
    let t83398 = t23384 * t23317;
    let t83408 = t23572 * t225;
    let t83420 = t6698 * t23587;
    let t83424 = t6688 * t3166;
    let t83435 = t23384 * t23399;
    let t83441 = t82573 * t6692;
    (t83368, t83398, t83408, t83420, t83424, t83435, t83441)
}
