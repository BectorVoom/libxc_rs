//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 448/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk448<F: Float>(t1578: F, t1561: F, t466: F, t1141: F, t1143: F, t220: F, t468: F) -> (F, F, F) {
    let t1579 = param_beta * t1578;
    let t1581 = t466 * t1561;
    let t1586 = t1141 * t1143 * t1581 + t1578 * t220 * t468;
    (t1579, t1581, t1586)
}
