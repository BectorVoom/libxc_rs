//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 806/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk806<F: Float>(t5552: F, t785: F, t1699: F, t792: F, t228: F, t64: F, t234: F, t339: F) -> (F, F, F, F) {
    let t5553 = t5552 * t785;
    let t5555 = t1699 * t792;
    let t5556 = 7.0 / 2304.0 * t5555;
    let t5557 = t228 * t64;
    let t5559 = t339 * t5557 * t234;
    (t5553, t5556, t5557, t5559)
}
