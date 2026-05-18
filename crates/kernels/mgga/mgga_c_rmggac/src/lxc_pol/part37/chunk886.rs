//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 886/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk886<F: Float>(t70052: F, t14148: F, t14150: F, t40717: F, t240: F, t4738: F, t574: F, t7351: F, t1614: F, t3065: F, t3928: F, t13839: F, t2044: F, t570: F, t7554: F) -> (F, F, F, F, F, F) {
    let t75874 = F::new(0.19863479950205658386e-4) * t70052;
    let t75876 = t14148 * t40717 * t14150;
    let t75881 = t14148 * t7351 * t574 * t240 * t4738;
    let t75886 = t3065 * t1614;
    let t75887 = t3928 * t75886;
    let t75892 = t13839 * t2044 * t7554 * t570;
    (t75874, t75876, t75881, t75886, t75887, t75892)
}
