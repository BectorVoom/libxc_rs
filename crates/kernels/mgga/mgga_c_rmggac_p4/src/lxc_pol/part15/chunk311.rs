//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 311/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk311<F: Float>(t1878: F, t1882: F, t1885: F, t206: F, t207: F, t600: F, t602: F) -> F {
    let t1888 = -t1878 * t207 - F::cast_from(12.0_f64) * t1882 * t206 + F::cast_from(3.0_f64) * t1885 * t206 + F::cast_from(6.0_f64) * t600 * t602;
    t1888
}
