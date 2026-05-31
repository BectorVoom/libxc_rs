//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1464/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1464<F: Float>(t108888: F, t108918: F, t109966: F, t109980: F, t105105: F, t107552: F, t107555: F, t107558: F, t107566: F, t107568: F, t107570: F, t107573: F, t107575: F, t107577: F, t107579: F, t107581: F, t107583: F, t107585: F, t1458: F, t20347: F, t2169: F, t22445: F, t22448: F, t24972: F, t27921: F, t5456: F, t5493: F, t577: F, t7423: F, t96334: F) -> (F, F) {
    let t109982 = t108888 + t108918 + t109966 + t109980;
    let t110002 = t107552 + F::cast_from(0.45e1_f64) * t109982 * t577 + t107555 + t107558 + F::cast_from(0.135e2_f64) * t7423 * t20347 + t107566 + t107568 + t107570 + F::cast_from(81.0_f64) * t24972 * t22448 + F::cast_from(0.405e2_f64) * t105105 * t1458 + F::cast_from(0.405e2_f64) * t27921 * t5493 + t107573 + t107575 + t107577 + t107579 + t107581 + F::cast_from(81.0_f64) * t96334 * t5456 + t107583 + F::cast_from(27.0_f64) * t2169 * t22445 + t107585;
    (t109982, t110002)
}
