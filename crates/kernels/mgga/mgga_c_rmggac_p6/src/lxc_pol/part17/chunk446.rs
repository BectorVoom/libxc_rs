//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 446/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk446<F: Float>(t1302: F, t1338: F, t1343: F, t270: F, t271: F, t4710: F, t4712: F, t4720: F, t4781: F, t4787: F, t4789: F, t71: F) -> F {
    let t4792 = F::cast_from(0.1714584e0_f64) * t4710 - F::cast_from(0.1714584e0_f64) * t4712 * t1302 + F::cast_from(0.285764e-1_f64) * t4720 + F::cast_from(0.285764e-1_f64) * t4781 * t271 - F::cast_from(0.857292e-1_f64) * t1338 * t1343 * t270 + F::cast_from(0.571528e-1_f64) * t4787 * t4789;
    let t4793 = t4792 * t71;
    t4793
}
