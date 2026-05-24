//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 471/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk471<F: Float>(t13809: F, t2019: F, t2012: F, t2131: F, t2010: F, t271: F, t3076: F) -> (F, F, F, F) {
    let t13810 = t2019 * t13809;
    let t13812 = t2012 * t2131;
    let t13813 = t2010 * t13812;
    let t13815 = t3076 * t271;
    (t13810, t13812, t13813, t13815)
}
