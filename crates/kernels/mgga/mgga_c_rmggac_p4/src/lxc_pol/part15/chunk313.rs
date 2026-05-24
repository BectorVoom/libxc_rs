//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 313/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk313<F: Float>(t1174: F, t1459: F, t1857: F, t1860: F, t1864: F, t1907: F, t228: F, t462: F, t598: F) -> F {
    let t1910 = t1857 * t228 + t1860 * t228 + t598 * t1459 / F::new(2.0) - F::new(5.0) / F::new(16.0) * t1174 * t1864 + t462 * t1907 / F::new(4.0);
    t1910
}
