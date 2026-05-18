//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 754/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk754<F: Float>(t14585: F, t7279: F, t290: F, t70901: F, t2010: F, t7755: F, t31: F, t702: F, t640: F, t7553: F, t7555: F, t2012: F, t7349: F) -> (F, F, F, F, F, F) {
    let t72147 = t14585 * t7279;
    let t72162 = t290 * t70901;
    let t72164 = t2010 * t7755 * t72162;
    let t72166 = t702 * t31;
    let t72167 = t640 * t72166;
    let t72169 = t7553 * t7555 * t72167;
    let t72170 = F::new(0.43368970657079495312e-4) * t72169;
    let t72171 = t290 * t72166;
    let t72173 = t7349 * t2012 * t72171;
    (t72147, t72162, t72164, t72170, t72171, t72173)
}
