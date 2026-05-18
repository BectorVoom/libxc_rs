//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 565/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk565<F: Float>(t2164: F, t702: F, t638: F, t639: F, t2231: F, t640: F, t290: F, t3207: F) -> (F, F, F, F, F) {
    let t14559 = t2164 * t702;
    let t14561 = t638 * t639 * t14559;
    let t14562 = F::new(0.15243824895787514157e-3) * t14561;
    let t14563 = t640 * t2231;
    let t14565 = t638 * t639 * t14563;
    let t14566 = F::new(0.15243824895787514157e-3) * t14565;
    let t14567 = t290 * t3207;
    (t14559, t14562, t14563, t14566, t14567)
}
