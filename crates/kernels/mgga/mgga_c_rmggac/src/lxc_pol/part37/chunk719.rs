//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 719/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk719<F: Float>(t13839: F, t2044: F, t352: F, t7554: F, t333: F, t7273: F, t14362: F, t1993: F, t13868: F, t13797: F, t14077: F, t7282: F) -> (F, F, F, F, F) {
    let t70194 = t13839 * t2044 * t7554 * t352;
    let t70195 = F::cast_from(0.16566831523319392754e-1_f64) * t70194;
    let t70198 = t7273 * t2044 * t7554 * t333;
    let t70207 = t1993 * t14362;
    let t70208 = t70207 * t13868;
    let t70211 = t7282 * t14077 * t13797;
    (t70195, t70198, t70207, t70208, t70211)
}
