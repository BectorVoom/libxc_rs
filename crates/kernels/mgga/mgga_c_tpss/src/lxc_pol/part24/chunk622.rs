//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 622/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk622<F: Float>(t108: F, t2: F, t555: F, t105: F, t1325: F, t1327: F, t3515: F, t3519: F, t3525: F, t631: F, t637: F, t97: F) -> (F, F) {
    let t3528 = t108 * t2;
    let t3529 = t3528 * t555;
    let t3532 = -25.0 / 9.0 * t631 * t1325 + 10.0 / 9.0 * t97 * t3515 + 5.0 / 3.0 * t97 * t3519 - 25.0 / 9.0 * t1327 * t637 + 10.0 / 9.0 * t105 * t3525 - 5.0 / 3.0 * t105 * t3529;
    (t3529, t3532)
}
