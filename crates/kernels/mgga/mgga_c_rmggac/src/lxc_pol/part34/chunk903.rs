//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 903/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk903<F: Float>(t76132: F, t71804: F, t76103: F, t76108: F, t78486: F, t78487: F, t78488: F, t78491: F, t78493: F, t78495: F, t78497: F, t78498: F, t78499: F, t78500: F, t78501: F, t78502: F, t78503: F) -> (F,) {
    let t78504 = 0.85129199786595678799e-5 * t76132;
    let t78505 = -t78486 + t78487 - t71804 - t78488 - 0.58171619854173713846e-5 * t76103 - 0.21814357445315142692e-4 * t76108 - t78491 + t78493 - t78495 - t78497 + t78498 + t78499 + t78500 + t78501 - t78502 - t78503 + t78504;
    (t78505,)
}
