//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 787/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk787<F: Float>(t10475: F, t10478: F, t10472: F, t3131: F, t360: F, t376: F, t676: F, t2928: F, t320: F, t10294: F, t268: F, t271: F, t6546: F, t154: F, t3061: F, t276: F, t285: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10482 = t3131 * t360;
    let t10508 = t676 * t376;
    let t10523 = 1.0 / t2928 / t320;
    let t10542 = 0.36793333333333333333e0 * t10294;
    let t10544 = t268 * t6546 * t271;
    let t10545 = 0.93932222222222222223e0 * t10544;
    let t10564 = t154 * t3061;
    let t10577 = 28.0 / 27.0 * t10544;
    let t10595 = 1.0 / t276 / t285 / 4.0;
    (t10480, t10482, t10508, t10523, t10542, t10544, t10545, t10564, t10577, t10595)
}
