//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 128/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk128<F: Float>(t131: F, t534: F, t521: F, t90: F, t95: F, t101: F, t102: F, t320: F, t87: F, t98: F, rho1: F, tau1: F) -> (F, F, F, F, F, F) {
    let t535 = t534 * t131;
    let t537 = t521 / F::new(2.0);
    let t538 = t90 * t537;
    let t541 = rho1 * rho1;
    let t543 = F::new(1.0) / t95 / t541;
    let t544 = tau1 * t543;
    let t547 = -t537;
    let t548 = t101 * t547;
    let t551 = F::new(10.0) / F::new(3.0) * t87 * t538 - F::new(10.0) / F::new(3.0) * t544 * t102 + F::new(10.0) / F::new(3.0) * t98 * t548 + t320;
    (t535, t537, t538, t544, t547, t551)
}
