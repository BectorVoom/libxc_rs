//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 65/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk65<F: Float>(t123: F, t126: F, t129: F, t136: F) -> (F, F, F) {
    let t138 = F::cast_from(0.379785e1_f64) * t126 + F::cast_from(0.8969e0_f64) * t123 + F::cast_from(0.204775e0_f64) * t129 + F::cast_from(0.123235e0_f64) * t136;
    let t141 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t138;
    let t142 = F::ln(t141);
    (t138, t141, t142)
}
