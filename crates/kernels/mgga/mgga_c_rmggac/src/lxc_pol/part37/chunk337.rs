//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 337/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk337<F: Float>(t22: F, t698: F, t656: F, t3091: F, t3100: F, t3103: F, t3197: F, t3199: F, t3200: F) -> (F, F, F) {
    let t3224 = t698 * t22;
    let t3225 = t3224 * t656;
    let t3281 = t3197 - F::new(0.34093327067806677162e-2) * t3091 + t3199 + t3200 - F::new(0.9072038638458063915e-4) * t3100 + F::new(0.24108102678124669849e-4) * t3103;
    (t3224, t3225, t3281)
}
