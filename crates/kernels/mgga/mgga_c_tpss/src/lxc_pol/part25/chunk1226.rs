//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1226/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1226<F: Float>(t42667: F, t5784: F, t1792: F, t18666: F, t19388: F, t19396: F, t20246: F, t5489: F, t6304: F, t67429: F, t67431: F, t67433: F, t67436: F, t67440: F, t67451: F, t67454: F, t69097: F, t69165: F) -> (F,) {
    let t71490 = t42667 * t5784;
    let t71499 = 10.0 * t18666 * t69097 - 5.0 / 3.0 * t71490 * t5489 + t67429 + t67431 + t67433 + t67436 + t67440 - t67451 - t67454 - 2.0 / 3.0 * t69165 * t1792 - 10.0 / 3.0 * t20246 * t19388 - 4.0 / 3.0 * t19396 * t6304;
    (t71499,)
}
