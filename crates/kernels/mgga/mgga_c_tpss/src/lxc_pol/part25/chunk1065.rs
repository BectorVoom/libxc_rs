//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1065/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1065<F: Float>(t11004: F, t11051: F, t11179: F, t11188: F, t14551: F, t14553: F, t14556: F, t14559: F, t14561: F, t14564: F, t8797: F, t10994: F, t14454: F, t14459: F, t14462: F, t14466: F, t14471: F, t14475: F, t14479: F, t14484: F, t14489: F, t14492: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t14539: F, t14541: F, t14610: F, t8796: F) -> F {
    let t14630 = -t8797 + F::new(0.19419375e1) * t14551 - F::new(0.258925e1) * t14553 - F::new(0.1294625e1) * t14556 - F::new(0.412621875e-1) * t14559 + F::new(0.16504875e0) * t14561 + F::new(0.82524375e-1) * t14564 - t11179 + F::new(0.36793333333333333333e-1) * t11051 + t11188 - F::new(0.40256666666666666668e0) * t11004;
    let t14632 = -F::new(0.82785e-1) * t14454 + F::new(0.12077e1) * t14459 + F::new(0.16557e0) * t14462 - F::new(0.5519e-1) * t14466 - F::new(0.36793333333333333333e-1) * t14471 - F::new(0.49671e0) * t14475 + F::new(0.33114e0) * t14479 + F::new(0.16557e0) * t14484 - F::new(0.27595e-1) * t14489 - F::new(0.301925e0) * t14492 + t14610 - F::new(0.18396666666666666667e0) * t10994 + F::new(0.16504875e0) * t14539 + F::new(0.258925e1) * t14541 - F::new(0.33547222222222222222e0) * t14517 - F::new(0.40256666666666666666e0) * t14521 - F::new(0.181155e1) * t14525 + F::new(0.12077e1) * t14528 - F::new(0.20128333333333333333e0) * t14532 + F::new(0.60385e0) * t14535 - t8796 + t14630;
    t14632
}
