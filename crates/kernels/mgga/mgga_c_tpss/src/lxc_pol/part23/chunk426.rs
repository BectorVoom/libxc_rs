//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 426/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk426<F: Float>(t1425: F, t846: F, t1409: F, t870: F, t1416: F, t1419: F, t1422: F, t879: F, t882: F, t885: F) -> (F, F, F, F) {
    let t1427 = 1.0 * t846 * t1425;
    let t1429 = -t870 - 0.17123333333333333333e-1 * t1409;
    let t1436 = 0.3529725e1 * t1416 - t879 - 0.516475e0 * t1409 + 0.6311625e0 * t1419 - t882 - 0.104195e0 * t1422;
    let t1437 = t1436 * t885;
    (t1427, t1429, t1436, t1437)
}
