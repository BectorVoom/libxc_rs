//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 816/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk816<F: Float>(t2393: F, t374: F, t376: F, t370: F, t3158: F, t964: F, t10335: F, t221: F, t339: F, t3069: F, t3180: F, t3036: F, t67: F, t3067: F, t3186: F, t3062: F, t820: F) -> (F, F, F, F, F, F, F, F) {
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / 10368.0;
    let t10381 = t964 * t3158;
    let t10383 = t221 * t10335;
    let t10385 = 5.0 / 1296.0 * t339 * t10383;
    let t10390 = t3180 * t3069;
    let t10401 = t3036 * t67;
    let t10402 = t3067 * t10401;
    let t10403 = t3186 * t10402;
    let t10408 = t820 * t3062;
    (t10377, t10381, t10385, t10390, t10401, t10402, t10403, t10408)
}
