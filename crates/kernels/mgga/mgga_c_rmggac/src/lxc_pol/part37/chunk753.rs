//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 753/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk753<F: Float>(t14935: F, t874: F, t70188: F, t70271: F, t70316: F, t69287: F, t3281: F, t4616: F, t70610: F, t13964: F, t14065: F, t14092: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t73450 = t874 * t14935;
    let t73454 = F::new(0.46328831667894726561e-5) * t70188;
    let t73480 = F::new(0.65053455985619242964e-5) * t70271;
    let t73484 = F::new(0.65053455985619242964e-5) * t70316;
    let t73536 = F::new(0.30643330512125015891e-2) * t69287;
    let t73569 = t4616 * t3281;
    let t73624 = F::new(0.65053455985619242964e-5) * t70610;
    let t73645 = F::new(0.13010691197123848593e-4) * t13964;
    let t73658 = F::new(0.58171619854173713844e-4) * t14065;
    let t73659 = F::new(0.114000641766744825e-6) * t14092;
    (t73450, t73454, t73480, t73484, t73536, t73569, t73624, t73645, t73658, t73659)
}
