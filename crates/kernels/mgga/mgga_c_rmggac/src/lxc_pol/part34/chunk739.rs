//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 739/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk739<F: Float>(t69276: F, t75318: F, t75321: F, t15093: F, t2048: F, t25640: F, t74973: F, t3826: F, t75302: F, t1614: F, t3046: F, t3851: F, t3839: F, t75373: F, t75298: F, t325: F, t551: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t75388 = t69276 * t75318;
    let t75390 = t69276 * t75321;
    let t75393 = t15093 * t2048;
    let t75395 = t25640 * t74973;
    let t75397 = t3826 * t75302;
    let t75399 = t3046 * t1614;
    let t75400 = t3851 * t75399;
    let t75402 = t3851 * t75302;
    let t75405 = t3826 * t75399;
    let t75407 = t3839 * t75373;
    let t75409 = t3839 * t75298;
    let t75411 = t551 * t325;
    (t75388, t75390, t75393, t75395, t75397, t75399, t75400, t75402, t75405, t75407, t75409, t75411)
}
