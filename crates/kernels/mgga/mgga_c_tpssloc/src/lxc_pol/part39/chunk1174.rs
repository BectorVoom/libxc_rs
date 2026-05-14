//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1174/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1174<F: Float>(t4025: F, t671: F, t1441: F, t2363: F, t1395: F, t1453: F, t2332: F, t4067: F, t666: F, t2358: F, t4072: F, t649: F, t12813: F, t88: F, t1458: F, t2311: F) -> (F, F, F, F, F, F, F, F, F) {
    let t55934 = t4025 * t671;
    let t55962 = t1441 * t2363;
    let t66940 = t1395 * t671;
    let t86592 = t1453 * t2332;
    let t86595 = t4067 * t666;
    let t86598 = t1453 * t2358;
    let t90370 = t649 * t4072;
    let t90375 = t88 * t12813;
    let t90381 = t2311 * t1458;
    (t55934, t55962, t66940, t86592, t86595, t86598, t90370, t90375, t90381)
}
