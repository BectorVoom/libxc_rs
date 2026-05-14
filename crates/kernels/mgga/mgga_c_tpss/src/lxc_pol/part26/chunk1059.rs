//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1059/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1059<F: Float>(t10994: F, t14454: F, t14459: F, t14462: F, t14466: F, t14471: F, t14475: F, t14479: F, t14484: F, t14489: F, t14492: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t14539: F, t14541: F, t14770: F, t14790: F, t8871: F) -> (F,) {
    let t14792 = -0.104195e0 * t14454 + 0.20659e1 * t14459 + 0.20839e0 * t14462 - 0.69463333333333333334e-1 * t14466 - 0.46308888888888888889e-1 * t14471 - 0.62517e0 * t14475 + 0.41678e0 * t14479 + 0.20839e0 * t14484 - 0.34731666666666666667e-1 * t14489 - 0.516475e0 * t14492 + t14770 - 0.23154444444444444445e0 * t10994 + 0.6311625e0 * t14539 + 0.3529725e1 * t14541 - 0.57386111111111111112e0 * t14517 - 0.68863333333333333334e0 * t14521 - 0.309885e1 * t14525 + 0.20659e1 * t14528 - 0.34431666666666666667e0 * t14532 + 0.103295e1 * t14535 - t8871 + t14790;
    (t14792,)
}
