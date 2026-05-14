//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 944/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk944<F: Float>(t352: F, t5148: F, t76372: F, t78114: F, t78115: F, t78116: F, t78117: F, t78119: F, t78120: F, t78189: F, t78194: F, t78199: F, t78201: F, t78203: F, t78205: F, t80452: F) -> (F,) {
    let t80489 = -t76372 + t78114 + t78115 + t78116 - t78117 - t78119 - t78120 - t78189 - t78194 - 0.11974241701863808564e0 * t5148 * t80452 * t352 + t78199 + t78201 - t78203 + t78205;
    (t80489,)
}
