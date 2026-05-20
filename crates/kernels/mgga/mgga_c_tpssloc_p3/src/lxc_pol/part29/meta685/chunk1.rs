//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2339/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2339<F: Float>(t2110: F, t24505: F, t24508: F, t26070: F, t26073: F, t26076: F, t7256: F, t7259: F, t7435: F, t90150: F, t90153: F, t90160: F, t90343: F) -> F {
    let t96021 = F::new(2.0) / F::new(3.0) * t90343 * t2110 + F::new(2.0) / F::new(3.0) * t26070 * t7256 + F::new(2.0) / F::new(3.0) * t26070 * t7259 + t90150 * t2110 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t90153 * t2110 + F::new(2.0) / F::new(3.0) * t26073 * t7256 + F::new(2.0) / F::new(3.0) * t26073 * t7259 + t90160 * t2110 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t26076 * t7256 + F::new(2.0) / F::new(3.0) * t26076 * t7259 + t7435 * t24505 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t7435 * t24508;
    t96021
}
