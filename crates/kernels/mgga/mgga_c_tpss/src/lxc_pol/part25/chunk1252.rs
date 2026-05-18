//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1252/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1252<F: Float>(t21608: F, t1805: F, t4783: F, t18000: F, t18770: F, t21312: F, t1395: F, t6337: F, t5572: F, t4799: F, t4715: F, t18021: F, t2162: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21609 = param_beta * t21608;
    let t21623 = t1805 * t4783;
    let t21624 = t18000 * t21623;
    let t21627 = t18770 * t21312;
    let t21630 = t6337 * t1395;
    let t21631 = t5572 * t21630;
    let t21634 = t1805 * t4799;
    let t21635 = t5572 * t21634;
    let t21638 = t1805 * t4715;
    let t21640 = t18021 * t21638 * t2162;
    (t21609, t21623, t21624, t21627, t21630, t21631, t21634, t21635, t21638, t21640)
}
