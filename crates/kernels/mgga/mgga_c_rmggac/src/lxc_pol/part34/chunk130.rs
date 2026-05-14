//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 130/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk130<F: Float>(t361: F, t380: F, t383: F, t387: F, t423: F, t430: F, t435: F, t579: F, t581: F, t198: F, t454: F, t589: F) -> (F, F) {
    let t592 = t361 + t380 - t383 - t387 + t579 + t423 + t581 - t430 - t435;
    let t597 = -0.32163648644302209643e2 * t592 * t198 + 0.96490945932906628929e2 * t454 * t589;
    (t592, t597)
}
