//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 969/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk969<F: Float>(t11161: F, t11170: F, t11197: F, t11200: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F, t11314: F, t11317: F) -> F {
    let t11328 = -t11314 - F::new(0.52945875e1) * t11197 + F::new(0.94674375e0) * t11200 - t11317 + F::new(0.62517e0) * t11206 + F::new(0.104195e0) * t11209 + F::new(0.34731666666666666667e0) * t11211 + F::new(0.69463333333333333335e-1) * t11213 - F::new(0.41678000000000000001e0) * t11215 - F::new(0.20839e0) * t11217 + F::new(0.46308888888888888889e-1) * t11221 - F::new(0.20839e0) * t11224 - F::new(0.103295e1) * t11161 + F::new(0.309885e1) * t11170;
    t11328
}
