//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1231/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1231<F: Float>(t31832: F, t7754: F, t8689: F, t8944: F, t26164: F, t24994: F, t24996: F, t120108: F, t120111: F, t120114: F, t120171: F, t120173: F, t120176: F, t120177: F, t120658: F, t120659: F, t120663: F) -> (F,) {
    let t123193 = t31832 * t7754;
    let t123194 = t8689 * t8944;
    let t123195 = t123194 * t26164;
    let t123198 = t8689 * t24994;
    let t123199 = t123198 * t24996;
    let t123201 = -2.0 * t120108 + t123193 - t120111 - t120114 + t120171 + 2.0 * t123195 + 6.0 * t120173 - t120176 + t120177 + t120658 - t120659 + 6.0 * t123199 + t120663;
    (t123201,)
}
