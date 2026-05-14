//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 431/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk431<F: Float>(t1407: F, t835: F, t128: F, t834: F, t285: F, t833: F) -> (F, F, F, F, F) {
    let t1408 = t835 * t1407;
    let t1409 = t128 * t1408;
    let t1411 = -t834 - 0.17808333333333333333e-1 * t1409;
    let t1413 = 0.621814e-1 * t1411 * t285;
    let t1415 = -t833 / 3.0 - t1409 / 3.0;
    (t1408, t1409, t1411, t1413, t1415)
}
