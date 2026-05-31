//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 502/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk502<F: Float>(t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t510: F, t513: F, t574: F, t652: F) -> F {
    let t1851 = -t113 * t1774 - t1442 * t510 - F::cast_from(2.0_f64) * t1459 * t652 + t1778 * t574 + t1849 * t513;
    t1851
}
