//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 504/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk504<F: Float>(t14444: F, t352: F, t8940: F, t321: F, t5148: F, t36: F, t698: F) -> (F, F, F) {
    let t14445 = t14444 * t352;
    let t14447 = 0.11974241701863808564e0 * t8940 * t14445;
    let t14448 = t14444 * t321;
    let t14450 = 0.11974241701863808564e0 * t5148 * t14448;
    let t14451 = t698 * t36;
    (t14447, t14450, t14451)
}
