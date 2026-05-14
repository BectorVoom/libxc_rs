//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 837/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk837<F: Float>(t1294: F, t9713: F, t25: F, t526: F, t28: F, t528: F, t9722: F, t2528: F, t3691: F, t9919: F, t2663: F, t3814: F, t9905: F, t9892: F, t3826: F, t588: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12059 = 0.5848223622634646207e0 * t1294 * t9713;
    let t12061 = 1.0 / t526 / t25;
    let t12072 = 1.0 / t528 / t28;
    let t12087 = 0.10389515463408878255e3 * t1294 * t9722;
    let t12091 = t3691 * t2528;
    let t12094 = 0.35089341735807877242e1 * t1294 * t9919;
    let t12097 = t3814 * t2663;
    let t12103 = 0.35089341735807877242e1 * t1294 * t9905;
    let t12105 = 0.51947577317044391277e2 * t1294 * t9892;
    let t12106 = t588 * t3826;
    (t12059, t12061, t12072, t12087, t12091, t12094, t12097, t12103, t12105, t12106)
}
