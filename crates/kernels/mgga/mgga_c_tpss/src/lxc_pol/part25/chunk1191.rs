//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1191/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1191<F: Float>(t19414: F, t5791: F, t19417: F, t18660: F, t6080: F, t18670: F, t19388: F, t42178: F, t5784: F, t20275: F, t5483: F, t1675: F, t19380: F, t5790: F, t19345: F, t18350: F) -> (F, F, F, F, F, F, F, F, F) {
    let t67431 = 32.0 / 9.0 * t19414 * t5791;
    let t67433 = 32.0 / 9.0 * t19417 * t5791;
    let t67436 = 32.0 / 9.0 * t6080 * t18660;
    let t67440 = 80.0 / 9.0 * t18670 * t19388;
    let t67441 = t42178 * t5784;
    let t67451 = 16.0 / 9.0 * t5483 * t20275;
    let t67454 = 16.0 / 9.0 * t1675 * t5790 * t19380;
    let t67472 = t5790 * t19345;
    let t67474 = 160.0 / 9.0 * t18350 * t67472;
    (t67431, t67433, t67436, t67440, t67441, t67451, t67454, t67472, t67474)
}
