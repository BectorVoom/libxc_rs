//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 177/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk177<F: Float>(t547: F, t548: F, t10: F, t2: F, t17: F, t16: F, t3: F) -> (F, F, F, F, F) {
    let t550 = t547 * t548 + 1.0;
    let t551 = t10 * t2;
    let t553 = 2.0 * t551 * t17;
    let t554 = t16 * t3;
    let t555 = 1.0 / t554;
    (t550, t551, t553, t554, t555)
}
