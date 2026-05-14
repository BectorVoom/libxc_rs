//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 862/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk862<F: Float>(t10375: F, t370: F, t10250: F, t977: F, t3158: F, t964: F, t10335: F, t221: F, t339: F, t2955: F, t995: F, t3069: F, t3180: F, t3121: F, t884: F, t3071: F) -> (F, F, F, F, F, F, F, F) {
    let t10377 = t370 * t10375 / 10368.0;
    let t10378 = t977 * t10250;
    let t10381 = t964 * t3158;
    let t10383 = t221 * t10335;
    let t10385 = 5.0 / 1296.0 * t339 * t10383;
    let t10388 = t2955 * t995;
    let t10390 = t3180 * t3069;
    let t10393 = t3121 * t884;
    let t10394 = t3071 * t10393;
    (t10377, t10378, t10381, t10383, t10385, t10388, t10390, t10394)
}
