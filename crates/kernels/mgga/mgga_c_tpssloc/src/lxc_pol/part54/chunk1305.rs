//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1305/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1305<F: Float>(t112667: F, t112673: F, t1888: F, t23270: F, t25170: F, t112678: F, t112680: F, t112686: F, t112702: F, t30713: F, t4166: F, t30716: F) -> (F, F, F, F, F, F, F, F) {
    let t118499 = F::new(0.38381794893125283518e-1) * t112667;
    let t118500 = F::new(0.38381794893125283518e-1) * t112673;
    let t118503 = F::new(0.9869604401089358619e-1) * t1888 * t23270 * t25170;
    let t118506 = F::new(0.82246703342411321825e-2) * t112678;
    let t118518 = F::new(0.76763589786250567036e-1) * t112680;
    let t118523 = F::new(0.76763589786250567036e-1) * t112686;
    let t118526 = F::new(0.16449340668482264365e-1) * t112702;
    let t118532 = t4166 * t30713;
    let t118533 = t118532 * t30716;
    (t118499, t118500, t118503, t118506, t118518, t118523, t118526, t118533)
}
