//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 896/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk896<F: Float>(t33336: F, t7685: F, t28821: F, t8644: F, t128516: F, t128523: F, t128535: F, t128537: F, t128539: F, t128543: F, t128549: F, t1976: F, t2075: F, t24999: F, t27188: F, t27996: F, t28951: F, t28952: F, t29214: F, t29219: F, t29243: F, t33085: F, t6517: F, t652: F, t7472: F, t7802: F, t8450: F) -> (F,) {
    let t128551 = 2.0 * t7685 * t33336;
    let t128552 = t28821 * t8644;
    let t128553 = -2.0 * t1976 * t28951 * t652 - 2.0 * t2075 * t27996 - 4.0 * t24999 * t7802 - 4.0 * t27188 * t7472 - 2.0 * t28952 * t6517 - 2.0 * t29214 * t6517 - 4.0 * t29219 * t6517 + 2.0 * t29243 * t8450 - 4.0 * t33085 * t7802 - t128516 - t128523 - t128535 - t128537 - t128539 - t128543 + t128549 + t128551 - t128552;
    (t128553,)
}
