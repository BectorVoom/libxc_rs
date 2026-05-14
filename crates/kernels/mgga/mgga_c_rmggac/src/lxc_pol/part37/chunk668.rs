//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 668/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk668<F: Float>(t14125: F, t68440: F, t9045: F, t21713: F, t68422: F, t9050: F, t21719: F, t7248: F, t8503: F, t8507: F, t9188: F, t3352: F, t8807: F, t68386: F, t8811: F, t9205: F) -> (F, F, F, F, F, F, F, F) {
    let t73746 = t68440 * t14125 * t9045;
    let t73749 = t21713 * t68422 * t9050;
    let t73752 = t21719 * t7248 * t8503;
    let t73755 = t21719 * t9188 * t8507;
    let t73758 = t21719 * t3352 * t8807;
    let t73761 = t68386 * t3352 * t9045;
    let t73764 = t21713 * t14125 * t8811;
    let t73767 = t68386 * t3352 * t9205;
    (t73746, t73749, t73752, t73755, t73758, t73761, t73764, t73767)
}
