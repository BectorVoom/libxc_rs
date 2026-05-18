//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1056/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1056<F: Float>(t232: F, t4119: F, t2645: F, t4181: F, t16891: F, t2647: F, t13242: F, t5591: F, t13228: F, t13351: F, t13222: F, t16839: F, t9627: F) -> (F, F, F, F, F) {
    let t16912 = t232 * t4119;
    let t16914 = t2645 * t4181 * t16912;
    let t16918 = t2645 * t16891 * t2647;
    let t16924 = t2645 * t13242 * t5591;
    let t16927 = t13228 * t13351;
    let t16928 = t13222 * t16927;
    let t16932 = t2645 * t16839 * t9627;
    (t16914, t16918, t16924, t16928, t16932)
}
