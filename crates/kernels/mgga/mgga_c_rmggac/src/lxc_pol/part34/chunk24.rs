//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 24/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk24<F: Float>(t8: F, t50: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t51 = 1.0 / t8;
    let t52 = t50 * t51;
    let t53 = 1.0 + t52;
    let t54 = t53 <= zeta_threshold;
    let t55 = pow_1_3(zeta_threshold);
    let t56 = t55 * zeta_threshold;
    let t57 = pow_1_3(t53);
    (t51, t52, t53, t55, t56, t57)
}
