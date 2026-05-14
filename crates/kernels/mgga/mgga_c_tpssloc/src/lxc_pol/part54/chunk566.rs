//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 566/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk566<F: Float>(t1661: F, t3270: F, t1102: F, t3238: F, t3274: F, t4721: F, t4726: F, t4731: F, t4735: F, t1100: F, t3287: F, t1107: F, t1667: F, t699: F, t3297: F, t4724: F) -> (F, F, F, F, F, F) {
    let t4748 = t3270 * t1661;
    let t4749 = t4748 * t1102;
    let t4756 = t3274 - t3238 / 9.0 - t4721 / 9.0 - 2.0 / 9.0 * t4726 + 2.0 / 3.0 * t4731 + t4735 / 3.0;
    let t4757 = t1100 * t4756;
    let t4764 = t3287 * t1661;
    let t4765 = t4764 * t1102;
    let t4767 = t1107 * t4756;
    let t4770 = t699 * t1667;
    let t4772 = t3297 * t4724;
    (t4749, t4757, t4765, t4767, t4770, t4772)
}
