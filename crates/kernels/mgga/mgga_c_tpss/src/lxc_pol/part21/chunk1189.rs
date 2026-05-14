//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1189/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1189<F: Float>(t1250: F, t18464: F, t3350: F, t5728: F, t3354: F, t18435: F, t18438: F, t18440: F, t18442: F, t18447: F, t18452: F, t18455: F, t18457: F, t18459: F, t18462: F, t219: F, t5732: F) -> (F, F, F, F) {
    let t18465 = t18464 * t1250;
    let t18466 = 7.0 / 288.0 * t18465;
    let t18467 = t5728 * t3350;
    let t18469 = t5728 * t3354;
    let t18471 = t18435 + t18438 + t18440 / 16.0 - t18442 / 48.0 + t18447 / 768.0 + t18452 + t18455 / 192.0 - t18457 / 1536.0 - t18459 / 1536.0 + t18462 + t18466 + 5.0 / 384.0 * t18467 - t18469 / 384.0;
    let t18472 = param_beta * t18471;
    let t18474 = t5732 * t219;
    (t18465, t18471, t18472, t18474)
}
