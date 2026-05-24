//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 540/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk540<F: Float>(t132: F, t1341: F, t638: F, t7310: F, t2085: F, t2181: F, t33: F, t78: F, t271: F, t4765: F) -> (F, F, F, F, F, F) {
    let t7311 = t132 * t1341;
    let t7313 = t638 * t7310 * t7311;
    let t7318 = t2181 * t2085;
    let t7320 = t78 * t33;
    let t7321 = F::new(1.0) / t7320;
    let t7322 = t7321 * t271;
    let t7323 = t4765 * t7322;
    (t7311, t7313, t7318, t7321, t7322, t7323)
}
