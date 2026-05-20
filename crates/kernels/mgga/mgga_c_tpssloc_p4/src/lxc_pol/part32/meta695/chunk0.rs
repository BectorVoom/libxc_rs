//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2160/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2160<F: Float>(t28160: F, t6883: F, t19873: F, t26309: F, t19966: F, t6396: F, t80816: F, t19951: F, t22833: F, t19972: F, t19976: F, t5259: F, t91100: F) -> (F, F, F, F, F, F, F, F) {
    let t97200 = t6883 * t28160;
    let t97202 = t26309 * t19873;
    let t97204 = t26309 * t19966;
    let t97206 = t80816 * t6396;
    let t97208 = t22833 * t19951;
    let t97210 = t22833 * t19972;
    let t97212 = t22833 * t19976;
    let t97214 = t91100 * t5259;
    (t97200, t97202, t97204, t97206, t97208, t97210, t97212, t97214)
}
