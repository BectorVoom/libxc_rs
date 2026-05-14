//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 956/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk956<F: Float>(t1289: F, t7771: F, t2033: F, t3431: F, t7780: F, t2040: F, t10353: F, t1985: F, t1992: F, t3472: F, t3477: F, t581: F, t608: F, t612: F, t77: F) -> (F, F, F) {
    let t10388 = t7771 * t1289;
    let t10391 = t2033 * t3431;
    let t10398 = t7780 * t1289;
    let t10401 = t2040 * t3431;
    let t10408 = -280.0 / 27.0 * t10388 * t1985 + 56.0 / 9.0 * t10391 * t581 + 28.0 / 9.0 * t3472 * t1992 - 4.0 / 3.0 * t608 * t10353 + 280.0 / 27.0 * t10398 * t1985 + 56.0 / 9.0 * t10401 * t581 + 28.0 / 9.0 * t3477 * t1992 + 4.0 / 3.0 * t612 * t10353;
    let t10409 = t77 * t10408;
    let t10412 = t1992 * t1289;
    (t10408, t10409, t10412)
}
