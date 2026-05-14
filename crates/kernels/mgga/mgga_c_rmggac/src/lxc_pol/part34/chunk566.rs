//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 566/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk566<F: Float>(t1985: F, t305: F, t27: F, t618: F, t117: F, t15280: F, t3350: F, t8576: F) -> (F, F, F, F) {
    let t16504 = t1985 * t305;
    let t17695 = t27 * t618;
    let t17787 = t15280 * t117;
    let t17859 = t8576 * t3350;
    (t16504, t17695, t17787, t17859)
}
