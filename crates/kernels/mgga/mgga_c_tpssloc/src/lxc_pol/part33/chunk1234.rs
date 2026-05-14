//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1234/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1234<F: Float>(t109: F, t20347: F, t88: F, t1873: F, t28007: F, t7467: F, t28017: F, t7676: F, t20304: F, t81446: F, t22473: F, t75603: F, t20342: F, t6530: F, t81438: F, t86586: F, t96713: F, t96721: F) -> (F, F, F, F) {
    let t110 = 1.0 < t109;
    let t106935 = t88 * t20347;
    let t106937 = 2.0 * t106935 * t1873;
    let t106939 = 6.0 * t28007 * t7467;
    let t106941 = 6.0 * t7676 * t28017;
    let t106944 = t81446 * t20304;
    let t106946 = t22473 * t75603;
    let t106948 = t6530 * t20342;
    let t106951 = piecewise3(t110, 0.0, -t81438 - 11.0 / 3.0 * t86586 - 2.0 * t96713 + t96721 - 3.0 / 4.0 * t106944 + 3.0 / 4.0 * t106946 - t106948 / 8.0);
    (t106937, t106939, t106941, t106951)
}
