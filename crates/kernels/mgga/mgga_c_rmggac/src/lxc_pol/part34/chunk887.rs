//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 887/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk887<F: Float>(t78207: F, t8622: F, t118: F, t326: F, t338: F, t76414: F, t77488: F, t77525: F, t77720: F, t78119: F, t78120: F, t78184: F, t78189: F, t78194: F, t78199: F, t78201: F, t78203: F, t78205: F) -> (F,) {
    let t78208 = t78207 * t8622;
    let t78209 = 0.20455996240684006297e-1 * t78208;
    let t78210 = -t78119 - t78120 + 0.19957069503106347607e-1 * t118 * t338 * t78184 - t78189 - 0.59871208509319042821e-1 * t326 * t77720 - 0.59871208509319042821e-1 * t326 * t77525 - t78194 - 0.39914139006212695214e-1 * t118 * t77488 + t78199 + t78201 - t78203 + t78205 + t78209 - t76414;
    (t78210,)
}
