//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1263/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1263<F: Float>(t107571: F, t1873: F, t16524: F, t28896: F, t28899: F, t33185: F, t20162: F, t7467: F, t55388: F, t7769: F, t28893: F, t100911: F, t107545: F, t107552: F, t107555: F, t107558: F, t107566: F, t107568: F, t107570: F, t1458: F, t2022: F, t20347: F, t22445: F, t22448: F, t23880: F, t26523: F, t5456: F, t5493: F, t577: F, t7010: F, t86647: F) -> (F,) {
    let t107573 = 81.0 * t107571 * t1873;
    let t107575 = 162.0 * t16524 * t28896;
    let t107577 = 81.0 * t16524 * t28899;
    let t107579 = 81.0 * t33185 * t28899;
    let t107581 = 0.405e2 * t20162 * t7467;
    let t107583 = 81.0 * t55388 * t7769;
    let t107585 = 81.0 * t28893 * t7467;
    let t107588 = 27.0 * t2022 * t22445 + 0.45e1 * t107545 * t577 + 81.0 * t23880 * t22448 + t107552 + t107555 + t107558 + 81.0 * t86647 * t5456 + 0.405e2 * t26523 * t5493 + 0.135e2 * t7010 * t20347 + t107566 + t107568 + t107570 + t107573 + t107575 + t107577 + t107579 + t107581 + t107583 + t107585 + 0.405e2 * t100911 * t1458;
    (t107588,)
}
