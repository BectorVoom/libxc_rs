//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 866/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk866<F: Float>(t1779: F, t6243: F, t1270: F, t1625: F, t5708: F, t1760: F, t1630: F, t5716: F, t1642: F, t5721: F, t1646: F, t5728: F, t5715: F, t5725: F) -> (F, F, F, F, F) {
    let t6244 = t6243 * t1779;
    let t6245 = t1270 * t1625;
    let t6246 = t5708 * t6245;
    let t6248 = 3.0 * t1760 * t6246;
    let t6249 = t5716 * t1630;
    let t6251 = t5721 * t1642;
    let t6253 = t5728 * t1646;
    let t6255 = -t5715 - t6249 / 48.0 - t6251 / 1536.0 - t5725 - t6253 / 384.0;
    (t6244, t6245, t6246, t6248, t6255)
}
