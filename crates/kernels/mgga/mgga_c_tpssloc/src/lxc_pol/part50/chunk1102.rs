//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1102/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1102<F: Float>(t12571: F, t31003: F, t4021: F, t8307: F, t8513: F, t113845: F, t113848: F, t113851: F, t119944: F, t119948: F, t119952: F, t119955: F, t119965: F, t119971: F, t119975: F, t119978: F, t119981: F, t2240: F, t31004: F, t31006: F, t31017: F, t31019: F, t31022: F, t31024: F, t33107: F, t33115: F, t33119: F, t6504: F, t8301: F, t8309: F) -> (F,) {
    let t119984 = t12571 * t31003;
    let t119990 = t8513 * t8307 * t4021;
    let t119993 = 5.0 / 36.0 * t31017 * t119944 - 5.0 / 12.0 * t31004 * t119948 + 5.0 / 36.0 * t31017 * t119952 + 5.0 / 144.0 * t119955 * t8309 + 5.0 / 72.0 * t113848 * t33115 + 5.0 / 72.0 * t2240 * t8301 * t6504 * t33115 + 5.0 / 72.0 * t31017 * t119965 + 5.0 / 72.0 * t113851 * t33119 + 5.0 / 72.0 * t31022 * t119971 + 5.0 / 72.0 * t31022 * t119975 - 5.0 / 24.0 * t119978 * t31006 + 5.0 / 72.0 * t119981 * t31019 + 5.0 / 72.0 * t119984 * t31024 - 5.0 / 24.0 * t113845 * t33107 - 5.0 / 24.0 * t31004 * t119990;
    (t119993,)
}
