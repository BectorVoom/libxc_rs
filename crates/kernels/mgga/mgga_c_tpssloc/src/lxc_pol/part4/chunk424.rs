//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 424/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk424<F: Float>(t1409: F, t55: F, t1414: F, t1420: F, t39: F, t51: F, t56: F, t627: F) -> (F, F) {
    let t1423 = t55 * t1409;
    let t1426 = 5.0 / 6.0 * t39 * t1414 - 8.0 / 3.0 * t1420 * t56 - 5.0 / 6.0 * t51 * t1423 + t627;
    (t1423, t1426)
}
