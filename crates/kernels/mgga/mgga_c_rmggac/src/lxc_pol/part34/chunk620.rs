//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 620/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk620<F: Float>(t4669: F, t69160: F, t14301: F, t25525: F, t14308: F, t3814: F, t25640: F, t3065: F, t14173: F, t3851: F, t2500: F, t68756: F, t128: F, t1330: F, t793: F, t14229: F, t7254: F) -> (F, F, F, F, F, F, F, F) {
    let t69469 = t4669 * t69160;
    let t69481 = t25525 * t14301;
    let t69484 = t3814 * t14308;
    let t69507 = t25640 * t3065;
    let t69511 = t3851 * t14173;
    let t69518 = t2500 * t68756;
    let t69521 = t793 * t128 * t1330;
    let t69568 = t7254 * t14229;
    (t69469, t69481, t69484, t69507, t69511, t69518, t69521, t69568)
}
