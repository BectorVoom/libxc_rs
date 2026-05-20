//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1396/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1396<F: Float>(t109: F, t28017: F, t7676: F, t20304: F, t81446: F, t22473: F, t75603: F, t20342: F, t6530: F, t81438: F, t86586: F, t96713: F, t96721: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t106941 = F::new(6.0) * t7676 * t28017;
    let t106944 = t81446 * t20304;
    let t106946 = t22473 * t75603;
    let t106948 = t6530 * t20342;
    let t106951 = piecewise3::<F>(t110, F::new(0.0), -t81438 - F::new(11.0) / F::new(3.0) * t86586 - F::new(2.0) * t96713 + t96721 - F::new(3.0) / F::new(4.0) * t106944 + F::new(3.0) / F::new(4.0) * t106946 - t106948 / F::new(8.0));
    (t106941, t106951)
}
