//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 665/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk665<F: Float>(t21719: F, t8507: F, t9188: F, t3352: F, t8807: F, t68386: F, t9045: F, t14125: F, t21713: F, t8811: F, t9205: F, t68455: F, t8667: F, t21709: F, t8830: F, t14117: F, t8835: F) -> (F, F, F, F, F, F, F, F) {
    let t73755 = t21719 * t9188 * t8507;
    let t73758 = t21719 * t3352 * t8807;
    let t73761 = t68386 * t3352 * t9045;
    let t73764 = t21713 * t14125 * t8811;
    let t73767 = t68386 * t3352 * t9205;
    let t73770 = t68455 * t14125 * t8667;
    let t73773 = t68455 * t21709 * t8830;
    let t73776 = t68455 * t14117 * t8835;
    (t73755, t73758, t73761, t73764, t73767, t73770, t73773, t73776)
}
