//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1161/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1161<F: Float>(t5: F, t19386: F, t19428: F, t117: F, t1600: F, t5531: F, t626: F, t2056: F, t6113: F, t3499: F, t1163: F, t6112: F, t1338: F, t5692: F, t5706: F, t6275: F, t1339: F, t17916: F, t19336: F, t19338: F, t19340: F, t485: F, t6096: F, t6117: F, t6228: F, t624: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t19430 = piecewise3(t8, 0.0, t19386 + t19428);
    let t19431 = t19430 * t117;
    let t19434 = t1600 * t5531;
    let t19436 = 2.0 * t626 * t19434;
    let t19438 = 2.0 * t2056 * t6113;
    let t19440 = 2.0 * t3499 * t6113;
    let t19441 = t1163 * t6112;
    let t19443 = 2.0 * t626 * t19441;
    let t19448 = t5692 * t1338;
    let t19452 = t5706 * t6275;
    let t19455 = -t1163 * t6096 - 2.0 * t1339 * t17916 - t19431 * t485 - 2.0 * t19448 * t626 - 2.0 * t2056 * t6117 - 2.0 * t3499 * t6117 - t6228 * t624 - t19336 - t19338 - t19340 - t19436 - t19438 - t19440 - t19443 + t19452;
    (t19430, t19431, t19434, t19441, t19448, t19455)
}
