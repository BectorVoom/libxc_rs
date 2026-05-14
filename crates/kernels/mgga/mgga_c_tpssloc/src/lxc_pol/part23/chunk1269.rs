//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1269/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1269<F: Float>(t381: F, t76722: F, t1058: F, t1060: F, t14608: F, t14618: F, t1615: F, t1625: F, t1630: F, t18086: F, t21594: F, t21614: F, t21617: F, t21644: F, t21650: F, t21653: F, t3186: F, t3188: F, t43503: F, t43505: F, t47857: F, t5937: F, t69924: F, t77485: F, t77806: F, t77826: F) -> (F, F) {
    let t77855 = t381 * t76722;
    let t77892 = 4.0 * t1058 * t1060 * t1615 * t21614 + 4.0 * t1058 * t1060 * t1625 * t21594 + t1058 * t1060 * t381 * t77485 + 24.0 * t21617 * t3186 * t77806 + 6.0 * t3186 * t3188 * t77855 - t43503 * t43505 * t77826 - 12.0 * t14608 * t21653 + 24.0 * t14618 * t21644 + 4.0 * t1630 * t69924 + 6.0 * t18086 * t5937 - 24.0 * t21650 * t47857;
    (t77855, t77892)
}
