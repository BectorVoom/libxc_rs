//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 761/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk761<F: Float>(t3352: F, t68386: F, t9205: F, t14125: F, t68455: F, t8667: F, t21709: F, t8830: F, t14117: F, t8835: F, t8842: F, t15208: F, t68922: F) -> (F, F, F, F, F, F) {
    let t73767 = t68386 * t3352 * t9205;
    let t73770 = t68455 * t14125 * t8667;
    let t73773 = t68455 * t21709 * t8830;
    let t73776 = t68455 * t14117 * t8835;
    let t73779 = t68455 * t14117 * t8842;
    let t73783 = t68922 * t15208;
    (t73767, t73770, t73773, t73776, t73779, t73783)
}
