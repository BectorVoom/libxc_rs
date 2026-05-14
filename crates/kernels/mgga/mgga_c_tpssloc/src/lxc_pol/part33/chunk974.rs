//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 974/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk974<F: Float>(t22152: F, t22202: F, t22267: F, t22325: F, t466: F, t1720: F, t6238: F, t1751: F, t6150: F, t1734: F, t1246: F, t22298: F, t491: F, t11915: F, t1932: F, t475: F) -> (F, F, F, F, F, F, F, F) {
    let t22327 = t22152 + t22202 + t22267 + t22325;
    let t22328 = t466 * t22327;
    let t22334 = t1720 * t6238;
    let t22337 = t6150 * t1751;
    let t22340 = t6238 * t1734;
    let t22341 = t22340 * t1246;
    let t22348 = t491 * t22298;
    let t22349 = t22348 * t11915;
    let t22354 = t1932 * t1734 * t475;
    (t22327, t22328, t22334, t22337, t22341, t22348, t22349, t22354)
}
