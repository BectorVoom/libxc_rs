//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1076/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1076<F: Float>(t75252: F, t69667: F, t73344: F, t75248: F, t75266: F, t75269: F, t77529: F, t77533: F, t77537: F, t77540: F, t77542: F, t77545: F, t77550: F, t77553: F, t77556: F, t77557: F, t77558: F) -> F {
    let t80244 = F::new(0.13469175824740901074e-6) * t75252;
    let t80247 = -t69667 - t73344 + t75248 + t77529 + t77533 + t77537 + t77540 + t80244 + t77542 + t77545 + F::new(0.58171619854173713844e-5) * t75266 - F::new(0.58171619854173713844e-5) * t75269 + t77550 - t77553 - t77556 - t77557 - t77558;
    t80247
}
