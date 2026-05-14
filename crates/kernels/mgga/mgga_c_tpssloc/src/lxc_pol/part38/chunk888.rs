//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 888/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk888<F: Float>(t376: F, t676: F, t1023: F, t248: F, t1020: F, t1017: F, t3087: F, t1015: F, t1012: F, t2928: F, t320: F, t10294: F, t268: F, t271: F, t6546: F, t2394: F, t885: F) -> (F, F, F, F, F, F, F, F) {
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10515 = t3087 * t1017;
    let t10516 = t1015 * t10515;
    let t10517 = t1012 * t10516;
    let t10523 = 1.0 / t2928 / t320;
    let t10542 = 0.36793333333333333333e0 * t10294;
    let t10544 = t268 * t6546 * t271;
    let t10545 = 0.93932222222222222223e0 * t10544;
    let t10556 = t2394 * t885;
    (t10508, t10511, t10517, t10523, t10542, t10544, t10545, t10556)
}
