//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 885/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk885<F: Float>(t15258: F, t16156: F, t21719: F, t9188: F, t9197: F, t15235: F, t68432: F, t21713: F, t68422: F, t8503: F, t21714: F, t8507: F) -> (F, F, F, F, F) {
    let t75820 = t16156 * t15258;
    let t75823 = t21719 * t9188 * t9197;
    let t75825 = t68432 * t15235;
    let t75828 = t21713 * t68422 * t8503;
    let t75831 = t21713 * t21714 * t8507;
    (t75820, t75823, t75825, t75828, t75831)
}
