//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1042/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1042<F: Float>(t10994: F, t14454: F, t14459: F, t14462: F, t14466: F, t14471: F, t14475: F, t14479: F, t14484: F, t14489: F, t14492: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t14539: F, t14541: F, t14610: F, t14630: F, t8796: F) -> (F,) {
    let t14632 = -0.82785e-1 * t14454 + 0.12077e1 * t14459 + 0.16557e0 * t14462 - 0.5519e-1 * t14466 - 0.36793333333333333333e-1 * t14471 - 0.49671e0 * t14475 + 0.33114e0 * t14479 + 0.16557e0 * t14484 - 0.27595e-1 * t14489 - 0.301925e0 * t14492 + t14610 - 0.18396666666666666667e0 * t10994 + 0.16504875e0 * t14539 + 0.258925e1 * t14541 - 0.33547222222222222222e0 * t14517 - 0.40256666666666666666e0 * t14521 - 0.181155e1 * t14525 + 0.12077e1 * t14528 - 0.20128333333333333333e0 * t14532 + 0.60385e0 * t14535 - t8796 + t14630;
    (t14632,)
}
