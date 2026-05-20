//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 916/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk916<F: Float>(t5392: F, t9321: F, t9330: F, t111: F, t5449: F, t5465: F, t626: F, t5464: F, t9365: F, t5489: F, t5468: F, t9384: F) -> (F, F, F, F, F, F, F) {
    let t19420 = t9321 * t5392;
    let t19430 = t9330 * t5392;
    let t19451 = t5449 * t111;
    let t19471 = t626 * t5465;
    let t19473 = t9365 * t5464;
    let t19480 = t626 * t5489;
    let t19488 = t9384 * t5468;
    (t19420, t19430, t19451, t19471, t19473, t19480, t19488)
}
