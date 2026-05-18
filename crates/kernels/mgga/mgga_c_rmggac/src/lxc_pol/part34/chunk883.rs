//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 883/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk883<F: Float>(t13862: F, t3120: F, t8581: F, t14107: F, t15280: F, t21708: F, t68422: F, t9212: F, t21714: F, t9217: F, t14125: F, t9105: F) -> (F, F, F, F, F) {
    let t75792 = t3120 * t13862 * t8581;
    let t75794 = t15280 * t14107;
    let t75797 = t21708 * t68422 * t9212;
    let t75800 = t21708 * t21714 * t9217;
    let t75803 = t21708 * t14125 * t9105;
    (t75792, t75794, t75797, t75800, t75803)
}
