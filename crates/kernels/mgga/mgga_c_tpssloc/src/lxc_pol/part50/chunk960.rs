//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 960/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk960<F: Float>(t5: F, t31003: F, t9239: F, t645: F, t8307: F, t8513: F, t33: F, t8303: F, t2240: F, t31: F, t607: F, t1862: F, t8301: F, t6504: F, t641: F, t79: F, t31000: F, t8309: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t31004 = t9239 * t31003;
    let t31005 = t8307 * t645;
    let t31006 = t8513 * t31005;
    let t31009 = t33 * t8303;
    let t31010 = t2240 * t31009;
    let t31011 = t8307 * t31;
    let t31013 = t8513 * t31011 * t607;
    let t31016 = t8301 * t1862;
    let t31017 = t2240 * t31016;
    let t31019 = t8513 * t8307 * t6504;
    let t31022 = t2240 * t31003;
    let t31024 = t8513 * t79 * t641;
    let t31028 = piecewise3(t8, 0.0, 5.0 / 144.0 * t31000 * t8309 - 5.0 / 24.0 * t31004 * t31006 - 5.0 / 36.0 * t31010 * t31013 + 5.0 / 72.0 * t31017 * t31019 + 5.0 / 72.0 * t31022 * t31024);
    (t31004, t31006, t31009, t31010, t31011, t31013, t31016, t31017, t31019, t31022, t31024, t31028)
}
