//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 104/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk104<F: Float>(t53: F, t60: F, t180: F, t243: F, t245: F, t426: F, t156: F, t171: F, t410: F, t416: F, t417: F, t195: F, t231: F, t57: F, t280: F, t62: F, t284: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t428 = t243 * t245 * t180;
    let t430 = 0.18311447306006545054e-3 * t426 * t428;
    let t431 = t156 * t171;
    let t433 = t410 * t416 * t417;
    let t435 = 0.5848223622634646207e0 * t431 * t433;
    let t436 = t195 * t231;
    let t437 = 1.0 / t57;
    let t440 = piecewise3(t54, 0.0, 2.0 / 3.0 * t437 * t280);
    let t441 = 1.0 / t62;
    let t444 = piecewise3(t61, 0.0, 2.0 / 3.0 * t441 * t284);
    let t446 = t440 / 2.0 + t444 / 2.0;
    (t428, t430, t431, t433, t435, t436, t437, t441, t446)
}
