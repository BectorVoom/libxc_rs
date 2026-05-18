//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 732/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk732<F: Float>(t699: F, t7617: F, t903: F, t2211: F, t739: F, t7840: F, t3180: F, t638: F, t7184: F, t14391: F, t16156: F, t68520: F) -> (F, F, F, F, F) {
    let t70809 = t903 * t699 * t7617;
    let t70812 = t739 * t2211 * t7840;
    let t70818 = t638 * t7184 * t3180;
    let t70819 = F::new(0.14905073231436680509e-2) * t70818;
    let t70867 = t16156 * t14391;
    let t70877 = F::new(0.29810146462873361016e-2) * t68520;
    (t70809, t70812, t70819, t70867, t70877)
}
