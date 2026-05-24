//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1062/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1062<F: Float>(t10994: F, t14454: F, t14459: F, t14462: F, t14466: F, t14471: F, t14475: F, t14479: F, t14484: F, t14489: F, t14492: F, t14510: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t14539: F, t14541: F, t14568: F, t8661: F) -> F {
    let t14570 = -F::cast_from(0.82156666666666666667e-1_f64) * t14454 + F::cast_from(0.11958666666666666667e1_f64) * t14459 + F::cast_from(0.16431333333333333333e0_f64) * t14462 - F::cast_from(0.54771111111111111112e-1_f64) * t14466 - F::cast_from(0.36514074074074074075e-1_f64) * t14471 - F::cast_from(0.49293999999999999999e0_f64) * t14475 + F::cast_from(0.32862666666666666666e0_f64) * t14479 + F::cast_from(0.16431333333333333333e0_f64) * t14484 - F::cast_from(0.27385555555555555556e-1_f64) * t14489 - F::cast_from(0.29896666666666666667e0_f64) * t14492 + t14510 - F::cast_from(0.18257037037037037037e0_f64) * t10994 + F::new(0.3071625e0) * t14539 + F::new(0.1898925e1) * t14541 - F::cast_from(0.33218518518518518518e0_f64) * t14517 - F::cast_from(0.39862222222222222222e0_f64) * t14521 - F::new(0.17938e1) * t14525 + F::cast_from(0.11958666666666666667e1_f64) * t14528 - F::cast_from(0.19931111111111111111e0_f64) * t14532 + F::cast_from(0.59793333333333333334e0_f64) * t14535 - t8661 + t14568;
    t14570
}
