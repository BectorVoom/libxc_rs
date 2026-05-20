//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1962/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1962<F: Float>(t2239: F, t5385: F, t111: F, t19449: F, t19644: F, t225: F, t20038: F, t20032: F, t20040: F, t19635: F, t20048: F, t1351: F, t6414: F) -> (F, F, F, F, F, F, F, F, F) {
    let t55921 = t5385 * t2239;
    let t55943 = t19449 * t111;
    let t56422 = t19644 * t225;
    let t56434 = t20038 * t225;
    let t56580 = t20032 * t225;
    let t56596 = t20040 * t225;
    let t56607 = t19635 * t225;
    let t56640 = t20048 * t225;
    let t56812 = t6414 * t1351;
    (t55921, t55943, t56422, t56434, t56580, t56596, t56607, t56640, t56812)
}
