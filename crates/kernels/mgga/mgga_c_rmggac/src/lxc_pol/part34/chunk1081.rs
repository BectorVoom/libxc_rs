//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1081/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1081<F: Float>(t71836: F, t1469: F, t34976: F, t39851: F, t699: F, t34975: F, t9145: F, t16503: F, t35039: F, t8420: F, t76504: F, t1664: F, t3207: F) -> (F, F, F, F, F, F) {
    let t78514 = F::new(0.39914139006212695213e-1) * t71836;
    let t78517 = t39851 * t34976 * t699 * t1469;
    let t78518 = F::new(0.85129199786595678796e-5) * t78517;
    let t78521 = t34975 * t34976 * t699 * t9145;
    let t78522 = F::new(0.53205749866622299248e-5) * t78521;
    let t78525 = t16503 * t35039 * t699 * t8420;
    let t78526 = F::new(0.42564599893297839398e-5) * t78525;
    let t78528 = F::new(0.1702583995731913576e-4) * t76504;
    let t78529 = t1664 * t3207;
    (t78514, t78518, t78522, t78526, t78528, t78529)
}
