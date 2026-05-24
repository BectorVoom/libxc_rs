//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1253/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1253<F: Float>(t1378: F, t226: F, t6337: F, t5577: F, t1805: F, t4758: F, t21638: F, t1708: F, t21608: F, t228: F, t1396: F, t1707: F, t18006: F, t1809: F, t19736: F, t20449: F, t21299: F, t21609: F, t21624: F, t21627: F, t21631: F, t21635: F, t21640: F, t253: F, t4784: F, t4800: F, t5571: F, t5834: F, t6135: F, t6343: F, t6348: F, t6351: F) -> (F, F, F, F, F) {
    let t21644 = t6337 * t1378 * t226;
    let t21645 = t5577 * t21644;
    let t21650 = t5577 * t1805 * t4758 * t226;
    let t21653 = t5577 * t21638 * t226;
    let t21656 = t1708 * t228 * t21608;
    let t21658 = -F::new(2.0) * t1396 * t20449 - t1707 * t21656 - F::new(4.0) * t18006 * t21627 - t1809 * t21299 + F::new(4.0) * t19736 * t6343 + F::new(2.0) * t19736 * t6348 + t21609 * t253 - F::new(6.0) * t21624 * t5571 + F::new(4.0) * t21631 * t5571 + F::new(2.0) * t21635 * t5571 - F::new(2.0) * t21640 * t5571 + F::new(2.0) * t21645 * t5571 + t21650 * t5571 + t21653 * t5571 + F::new(2.0) * t4784 * t5834 - t4800 * t5834 - F::new(2.0) * t6135 * t6351;
    (t21645, t21650, t21653, t21656, t21658)
}
