//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1065/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1065<F: Float>(t11004: F, t11051: F, t11179: F, t11188: F, t14551: F, t14553: F, t14556: F, t14559: F, t14561: F, t14564: F, t8797: F, t10994: F, t14454: F, t14459: F, t14462: F, t14466: F, t14471: F, t14475: F, t14479: F, t14484: F, t14489: F, t14492: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t14539: F, t14541: F, t14610: F, t8796: F) -> F {
    let t14630 = -t8797 + F::cast_from(0.19419375e1_f64) * t14551 - F::cast_from(0.258925e1_f64) * t14553 - F::cast_from(0.1294625e1_f64) * t14556 - F::cast_from(0.412621875e-1_f64) * t14559 + F::cast_from(0.16504875e0_f64) * t14561 + F::cast_from(0.82524375e-1_f64) * t14564 - t11179 + F::cast_from(0.36793333333333333333e-1_f64) * t11051 + t11188 - F::cast_from(0.40256666666666666668e0_f64) * t11004;
    let t14632 = -F::cast_from(0.82785e-1_f64) * t14454 + F::cast_from(0.12077e1_f64) * t14459 + F::cast_from(0.16557e0_f64) * t14462 - F::cast_from(0.5519e-1_f64) * t14466 - F::cast_from(0.36793333333333333333e-1_f64) * t14471 - F::cast_from(0.49671e0_f64) * t14475 + F::cast_from(0.33114e0_f64) * t14479 + F::cast_from(0.16557e0_f64) * t14484 - F::cast_from(0.27595e-1_f64) * t14489 - F::cast_from(0.301925e0_f64) * t14492 + t14610 - F::cast_from(0.18396666666666666667e0_f64) * t10994 + F::cast_from(0.16504875e0_f64) * t14539 + F::cast_from(0.258925e1_f64) * t14541 - F::cast_from(0.33547222222222222222e0_f64) * t14517 - F::cast_from(0.40256666666666666666e0_f64) * t14521 - F::cast_from(0.181155e1_f64) * t14525 + F::cast_from(0.12077e1_f64) * t14528 - F::cast_from(0.20128333333333333333e0_f64) * t14532 + F::cast_from(0.60385e0_f64) * t14535 - t8796 + t14630;
    t14632
}
