//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 750/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk750<F: Float>(t30: F, t1165: F, t1338: F, t2056: F, t3491: F, t3493: F, t3537: F, t4347: F, t645: F, t1170: F, t1614: F, t1173: F, t1288: F, t3282: F, t2: F, t490: F, t555: F, t580: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t4352 = 2.0 * t1165 * t3537 + 2.0 * t1338 * t2056 + 2.0 * t1338 * t4347 + 2.0 * t3493 * t645 + t3491;
    let t4356 = t1170 * t1614;
    let t4357 = 4.0 * t4356;
    let t4358 = t1173 * t1614;
    let t4359 = 4.0 * t4358;
    let t4360 = t3282 * t1288;
    let t4363 = t490 * t2;
    let t4367 = piecewise3(t31, 0.0, 4.0 / 9.0 * t4360 * t580 + 8.0 / 3.0 * t4363 * t555);
    (t4352, t4357, t4359, t4360, t4363, t4367)
}
