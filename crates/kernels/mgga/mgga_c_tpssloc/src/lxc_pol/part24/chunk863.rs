//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 863/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk863<F: Float>(t1023: F, t2780: F, t3071: F, t3036: F, t67: F, t3067: F, t3186: F, t3132: F, t884: F, t3062: F, t820: F, t2771: F, t3200: F, t3041: F, t2776: F, t3051: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10397 = t1023 * t2780;
    let t10398 = t3071 * t10397;
    let t10401 = t3036 * t67;
    let t10402 = t3067 * t10401;
    let t10403 = t3186 * t10402;
    let t10404 = t3132 * t884;
    let t10405 = t3071 * t10404;
    let t10408 = t820 * t3062;
    let t10409 = t1023 * t2771;
    let t10410 = t10408 * t10409;
    let t10413 = t3200 * t10402;
    let t10414 = t3041 * t884;
    let t10415 = t3071 * t10414;
    let t10418 = t2776 * t1023;
    let t10419 = t3071 * t10418;
    let t10422 = t820 * t3051;
    (t10398, t10401, t10403, t10405, t10410, t10413, t10415, t10419, t10422)
}
