//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 635/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk635<F: Float>(t2044: F, t333: F, t7273: F, t7554: F, t14362: F, t1993: F, t13868: F, t13797: F, t14077: F, t7282: F, t1986: F, t2090: F, t13957: F, t36292: F, t739: F, t14012: F, t14371: F) -> (F, F, F, F, F, F, F) {
    let t70198 = t7273 * t2044 * t7554 * t333;
    let t70207 = t1993 * t14362;
    let t70208 = t70207 * t13868;
    let t70211 = t7282 * t14077 * t13797;
    let t70212 = 0.10909864661698136691e0 * t70211;
    let t70221 = t1986 * t2090;
    let t70225 = t739 * t36292 * t13957;
    let t70229 = t14371 * t14012;
    (t70198, t70207, t70208, t70212, t70221, t70225, t70229)
}
