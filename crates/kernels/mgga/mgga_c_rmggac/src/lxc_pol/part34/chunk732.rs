//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 732/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk732<F: Float>(t68401: F, t68414: F, t1550: F, t699: F, t7596: F, t7617: F, t903: F, t2211: F, t739: F, t7840: F, t3180: F, t638: F, t7184: F) -> (F, F, F, F, F, F) {
    let t70797 = F::new(0.36765206969775206063e-5) * t68401;
    let t70799 = F::new(0.51300288795035171252e-6) * t68414;
    let t70806 = t1550 * t699 * t7596;
    let t70809 = t903 * t699 * t7617;
    let t70812 = t739 * t2211 * t7840;
    let t70818 = t638 * t7184 * t3180;
    (t70797, t70799, t70806, t70809, t70812, t70818)
}
