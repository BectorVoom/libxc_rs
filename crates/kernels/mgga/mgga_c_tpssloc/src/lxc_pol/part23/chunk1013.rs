//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1013/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1013<F: Float>(t22298: F, t475: F, t1214: F, t248: F, t11721: F, t3508: F, t11678: F, t11692: F, t11719: F, t11728: F, t11738: F, t15438: F, t15737: F, t15754: F, t1737: F, t1748: F, t19047: F, t19051: F, t19083: F, t19090: F, t19096: F, t22104: F, t22271: F, t22275: F, t22280: F, t22284: F, t22288: F, t3506: F, t3515: F, t3577: F, t467: F, t5005: F, t5024: F, t6207: F, t6211: F, t6227: F, t6232: F) -> (F, F, F, F, F, F, F) {
    let t22299 = t22298 * t475;
    let t22301 = t248 * t1214 * t22299;
    let t22307 = t22298 * t11721;
    let t22309 = t248 * t1214 * t22307;
    let t22312 = t22298 * t3508;
    let t22314 = t248 * t1214 * t22312;
    let t22325 = t19083 * t1748 / 144.0 + t3506 * t22271 / 512.0 - t3515 * t22275 / 1024.0 + t15754 / 432.0 - t11678 * t22280 / 768.0 + t11692 * t22284 / 1536.0 - t3577 * t22288 / 768.0 + 11.0 / 108.0 * t19090 + t15737 * t6227 / 512.0 - t15438 * t6232 / 1024.0 - t5005 * t6207 / 1536.0 + t11738 * t22301 / 3072.0 + t5024 * t6207 / 288.0 - t19096 / 1536.0 + t11719 * t22309 / 512.0 - t11728 * t22314 / 512.0 - t5005 * t6211 / 768.0 - 77.0 / 162.0 * t22104 * t467 + t19047 * t1737 / 1024.0 - t19051 * t1748 / 1536.0;
    (t22299, t22301, t22307, t22309, t22312, t22314, t22325)
}
