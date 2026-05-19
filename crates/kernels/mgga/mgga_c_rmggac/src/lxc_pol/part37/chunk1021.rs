//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1021/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1021<F: Float>(t76122: F, t76127: F, t76130: F, t76132: F, t71836: F, t1469: F, t34976: F, t39851: F, t699: F, t34975: F, t9145: F, t16503: F, t35039: F, t8420: F) -> (F, F, F, F, F, F, F, F) {
    let t78501 = F::cast_from(0.44903406381989282115e-1_f64) * t76122;
    let t78502 = F::cast_from(0.30487649791575028312e-3_f64) * t76127;
    let t78503 = F::cast_from(0.72042316457491791901e-3_f64) * t76130;
    let t78504 = F::cast_from(0.85129199786595678799e-5_f64) * t76132;
    let t78514 = F::cast_from(0.39914139006212695213e-1_f64) * t71836;
    let t78517 = t39851 * t34976 * t699 * t1469;
    let t78518 = F::cast_from(0.85129199786595678796e-5_f64) * t78517;
    let t78521 = t34975 * t34976 * t699 * t9145;
    let t78522 = F::cast_from(0.53205749866622299248e-5_f64) * t78521;
    let t78525 = t16503 * t35039 * t699 * t8420;
    (t78501, t78502, t78503, t78504, t78514, t78518, t78522, t78525)
}
