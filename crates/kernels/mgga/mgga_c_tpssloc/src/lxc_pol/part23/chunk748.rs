//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 748/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk748<F: Float>(t41: F, t42: F, t53: F, t54: F, t2585: F, t2769: F, t73: F, t3241: F, t76: F, t107: F, t655: F, t93: F, t94: F, t101: F, t102: F, t195: F, t40: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9287 = 1.0 / t42 / t41;
    let t9300 = 1.0 / t54 / t53;
    let t9311 = 1232.0 / 27.0 * t2585;
    let t9321 = 1.0 / t73 / t2769;
    let t9330 = 1.0 / t76 / t3241;
    let t9358 = 154.0 / 27.0 * t2585 * t107;
    let t9364 = t655 * t655;
    let t9365 = 1.0 / t9364;
    let t9383 = t94 * t93;
    let t9384 = 1.0 / t9383;
    let t9397 = t102 * t101;
    let t9398 = 1.0 / t9397;
    let t9427 = 1.0 / t195 / t40;
    (t9287, t9300, t9311, t9321, t9330, t9358, t9364, t9365, t9384, t9398, t9427)
}
