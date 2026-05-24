//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1073/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1073<F: Float>(t11004: F, t11051: F, t11319: F, t11328: F, t14551: F, t14553: F, t14556: F, t14559: F, t14561: F, t14564: F, t8872: F, t10994: F, t14454: F, t14459: F, t14462: F, t14466: F, t14471: F, t14475: F, t14479: F, t14484: F, t14489: F, t14492: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t14539: F, t14541: F, t14770: F, t8871: F) -> F {
    let t14790 = -t8872 + F::cast_from(0.264729375e1_f64) * t14551 - F::new(0.3529725e1) * t14553 - F::new(0.17648625e1) * t14556 - F::cast_from(0.157790625e0_f64) * t14559 + F::new(0.6311625e0) * t14561 + F::new(0.31558125e0) * t14564 - t11319 + F::cast_from(0.4630888888888888889e-1_f64) * t11051 + t11328 - F::cast_from(0.68863333333333333332e0_f64) * t11004;
    let t14792 = -F::new(0.104195e0) * t14454 + F::new(0.20659e1) * t14459 + F::new(0.20839e0) * t14462 - F::cast_from(0.69463333333333333334e-1_f64) * t14466 - F::cast_from(0.46308888888888888889e-1_f64) * t14471 - F::new(0.62517e0) * t14475 + F::new(0.41678e0) * t14479 + F::new(0.20839e0) * t14484 - F::cast_from(0.34731666666666666667e-1_f64) * t14489 - F::new(0.516475e0) * t14492 + t14770 - F::cast_from(0.23154444444444444445e0_f64) * t10994 + F::new(0.6311625e0) * t14539 + F::new(0.3529725e1) * t14541 - F::cast_from(0.57386111111111111112e0_f64) * t14517 - F::cast_from(0.68863333333333333334e0_f64) * t14521 - F::new(0.309885e1) * t14525 + F::new(0.20659e1) * t14528 - F::cast_from(0.34431666666666666667e0_f64) * t14532 + F::new(0.103295e1) * t14535 - t8871 + t14790;
    t14792
}
