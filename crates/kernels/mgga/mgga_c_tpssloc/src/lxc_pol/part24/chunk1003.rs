//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1003/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1003<F: Float>(t11292: F, t440: F, t11129: F, t3403: F, t11135: F, t11203: F, t11161: F, t11170: F, t11197: F, t11200: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F) -> (F, F, F) {
    let t11365 = t440 * t11292;
    let t11366 = t11129 * t3403;
    let t11369 = F::new(0.93932222222222222223e0) * t11135;
    let t11372 = F::new(0.36793333333333333333e0) * t11203;
    let t11383 = -t11369 - F::new(0.3883875e1) * t11197 + F::new(0.247573125e0) * t11200 - t11372 + F::new(0.49671e0) * t11206 + F::new(0.82785e-1) * t11209 + F::new(0.27595e0) * t11211 + F::new(0.5519e-1) * t11213 - F::new(0.33114e0) * t11215 - F::new(0.16557e0) * t11217 + F::new(0.36793333333333333333e-1) * t11221 - F::new(0.16557e0) * t11224 - F::new(0.60384999999999999999e0) * t11161 + F::new(0.181155e1) * t11170;
    (t11365, t11366, t11383)
}
