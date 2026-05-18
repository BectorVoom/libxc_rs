//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1159/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1159<F: Float>(t17691: F, t4987: F, t4582: F, t15654: F, t17686: F, t248: F, t3570: F, t6225: F, t3506: F, t1735: F, t4733: F, t3578: F) -> (F, F, F, F) {
    let t18341 = t4987 * t17691;
    let t18342 = t4582 * t18341;
    let t18345 = t15654 * t17686;
    let t18346 = t4582 * t18345;
    let t18356 = t248 * t3570 * t6225;
    let t18357 = t3506 * t18356;
    let t18359 = t1735 * t4733;
    let t18360 = t3578 * t18359;
    (t18342, t18346, t18357, t18360)
}
