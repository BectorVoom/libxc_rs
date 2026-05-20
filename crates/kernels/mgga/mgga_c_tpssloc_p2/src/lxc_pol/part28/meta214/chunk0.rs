//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 970/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk970<F: Float>(t1100: F, t4756: F, t1661: F, t3287: F, t1102: F, t1107: F, t1667: F, t699: F, t3297: F, t4724: F, t136: F, t1113: F, t4729: F) -> (F, F, F, F, F, F, F, F) {
    let t4757 = t1100 * t4756;
    let t4764 = t3287 * t1661;
    let t4765 = t4764 * t1102;
    let t4767 = t1107 * t4756;
    let t4770 = t699 * t1667;
    let t4772 = t3297 * t4724;
    let t4773 = t136 * t4772;
    let t4775 = t1113 * t4729;
    (t4757, t4764, t4765, t4767, t4770, t4772, t4773, t4775)
}
