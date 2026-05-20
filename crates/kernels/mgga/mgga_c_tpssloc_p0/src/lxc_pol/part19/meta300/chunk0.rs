//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1084/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1084<F: Float>(t1229: F, t3242: F, t3493: F, t3508: F, t11153: F, t3584: F, t1089: F, t1215: F, t607: F, t475: F, t1332: F, t5343: F) -> (F, F, F, F, F, F) {
    let t15615 = t1229 * t3242;
    let t15620 = t3508 * t3493;
    let t15654 = t3584 * t11153;
    let t15660 = t1215 * t1089;
    let t15661 = t15660 * t607;
    let t15707 = t607 * t1215;
    let t15708 = t15707 * t475;
    let t16033 = t1332 * t5343;
    (t15615, t15620, t15654, t15661, t15708, t16033)
}
