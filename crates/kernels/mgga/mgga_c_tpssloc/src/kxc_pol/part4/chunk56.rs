//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 56/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk56<F: Float>(t123: F, t126: F, t129: F, t136: F, t125: F) -> (F, F, F, F) {
    let t138 = F::new(0.379785e1) * t126 + F::new(0.8969e0) * t123 + F::new(0.204775e0) * t129 + F::new(0.123235e0) * t136;
    let t141 = F::new(1.0) + F::new(0.16081979498692535067e2) / t138;
    let t142 = f64::ln(t141);
    let t144 = F::new(0.621814e-1) * t125 * t142;
    (t138, t141, t142, t144)
}
