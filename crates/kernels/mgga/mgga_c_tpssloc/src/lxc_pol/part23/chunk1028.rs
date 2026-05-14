//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1028/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1028<F: Float>(t2386: F, t240: F, t2385: F, t2558: F, t686: F, t685: F, t120: F, t118: F, t123: F, t116: F, t268: F, t8705: F, t591: F, t9701: F, t2397: F, t693: F) -> (F, F, F, F, F, F, F, F) {
    let t39277 = t2386 * t240;
    let t39278 = t2385 * t39277;
    let t39280 = t686 * t2558;
    let t39281 = t685 * t39280;
    let t39283 = t120 * t2558;
    let t39284 = t118 * t39283;
    let t39286 = f64::powf(t123, -0.25e1);
    let t39289 = t39286 * t116 * t8705 * t268;
    let t39291 = t9701 * t591;
    let t39293 = t2397 * t39277;
    let t39295 = t693 * t39280;
    (t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295)
}
