//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 572/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk572<F: Float>(t14619: F, t14501: F, t739: F, t2069: F, t699: F, t1550: F, t2074: F, t903: F, t14105: F, t14473: F, t884: F, t1356: F, t14435: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14620 = F::new(0.34093327067806677161e-2) * t14619;
    let t14621 = t739 * t14501;
    let t14622 = F::new(0.59871208509319042821e-1) * t14621;
    let t14623 = t699 * t2069;
    let t14624 = t1550 * t14623;
    let t14625 = F::new(0.2993560425465952141e-1) * t14624;
    let t14626 = t699 * t2074;
    let t14627 = t903 * t14626;
    let t14628 = F::new(0.44903406381989282115e-1) * t14627;
    let t14630 = F::new(0.14967802127329760705e-1) * t14105;
    let t14633 = t884 * t14473;
    let t14634 = F::new(0.59871208509319042821e-1) * t14633;
    let t14635 = t1356 * t14435;
    (t14620, t14622, t14623, t14625, t14626, t14628, t14630, t14634, t14635)
}
