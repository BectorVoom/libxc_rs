//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1058/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1058<F: Float>(t78200: F, t72023: F, t8902: F, t72020: F, t8906: F, t22: F, t326: F, t8041: F, t8622: F, t118: F, t338: F, t76414: F, t77488: F, t77525: F, t77720: F, t78119: F, t78120: F, t78184: F, t78189: F, t78194: F, t78199: F) -> F {
    let t78201 = F::cast_from(0.27274661654245341728e-1_f64) * t78200;
    let t78202 = t72023 * t8902;
    let t78203 = F::cast_from(0.20455996240684006297e-1_f64) * t78202;
    let t78204 = t72020 * t8906;
    let t78205 = F::cast_from(0.27274661654245341729e-1_f64) * t78204;
    let t78207 = t326 * t8041 * t22;
    let t78208 = t78207 * t8622;
    let t78209 = F::cast_from(0.20455996240684006297e-1_f64) * t78208;
    let t78210 = -t78119 - t78120 + F::cast_from(0.19957069503106347607e-1_f64) * t118 * t338 * t78184 - t78189 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t77720 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t77525 - t78194 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t77488 + t78199 + t78201 - t78203 + t78205 + t78209 - t76414;
    t78210
}
