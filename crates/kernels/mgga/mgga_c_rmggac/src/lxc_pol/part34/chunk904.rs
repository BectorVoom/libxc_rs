//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 904/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk904<F: Float>(t71836: F, t1469: F, t34976: F, t39851: F, t699: F, t34975: F, t9145: F, t16503: F, t35039: F, t8420: F, t76504: F, t1664: F, t3207: F, t1356: F, t289: F, t70441: F, t70443: F, t70479: F, t71832: F, t71850: F, t76137: F, t76492: F, t76495: F, t76497: F, t76499: F, t78104: F) -> (F,) {
    let t78514 = 0.39914139006212695213e-1 * t71836;
    let t78517 = t39851 * t34976 * t699 * t1469;
    let t78518 = 0.85129199786595678796e-5 * t78517;
    let t78521 = t34975 * t34976 * t699 * t9145;
    let t78522 = 0.53205749866622299248e-5 * t78521;
    let t78525 = t16503 * t35039 * t699 * t8420;
    let t78526 = 0.42564599893297839398e-5 * t78525;
    let t78528 = 0.1702583995731913576e-4 * t76504;
    let t78529 = t1664 * t3207;
    let t78532 = 0.58171619854173713846e-5 * t76137 - t71832 + 0.39914139006212695214e-1 * t1356 * t78104 + 0.29085809927086856923e-4 * t70441 - 0.87257429781260570769e-4 * t70443 + 0.76860658247009135557e-5 * t76492 - t78514 - t78518 - t78522 - t78526 - t76495 - t76497 + t70479 - 0.35038612185802734376e-6 * t76499 - t78528 + t71850 - 0.2363e1 * t289 * t78529;
    (t78532,)
}
