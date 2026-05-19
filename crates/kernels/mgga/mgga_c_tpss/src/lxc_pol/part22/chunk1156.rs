//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1156/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1156<F: Float>(t10029: F, t1614: F, t3211: F, t3214: F, t1170: F, t4430: F, t1173: F, t4377: F, t724: F, t489: F, t10033: F, t2215: F, t4438: F) -> (F, F, F, F, F, F, F, F) {
    let t12907 = F::cast_from(0.11696447245269292414e1_f64) * t10029;
    let t12908 = t3211 * t1614;
    let t12909 = F::new(12.0) * t12908;
    let t12910 = t3214 * t1614;
    let t12911 = F::new(32.0) * t12910;
    let t12913 = F::new(8.0) * t1170 * t4430;
    let t12915 = F::new(8.0) * t1173 * t4430;
    let t12916 = t4377 * t724;
    let t12918 = F::new(2.0) * t489 * t12916;
    let t12919 = F::new(40.0) * t10033;
    let t12920 = t4438 * t2215;
    (t12907, t12909, t12911, t12913, t12915, t12918, t12919, t12920)
}
