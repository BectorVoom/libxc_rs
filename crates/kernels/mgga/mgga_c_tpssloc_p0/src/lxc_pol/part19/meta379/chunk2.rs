//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1418/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1418<F: Float>(t11270: F, t3259: F, t1094: F, t11274: F, t11278: F, t3262: F, t3311: F, t409: F, t3265: F, t11277: F, t11634: F, t3411: F) -> (F, F, F, F, F) {
    let t43963 = F::cast_from(4.0_f64) * t3259 * t11270;
    let t43964 = t1094 * t11274;
    let t43966 = F::cast_from(0.2069040516770936012e4_f64) * t43964 * t11278;
    let t43969 = t409 / t3311 / t3262;
    let t43970 = t3265 * t3265;
    let t43973 = F::cast_from(0.62071215503128080361e4_f64) * t43969 * t43970 * t11277;
    let t43975 = F::cast_from(0.20779030926817756511e3_f64) * t3411 * t11634;
    (t43963, t43966, t43970, t43973, t43975)
}
