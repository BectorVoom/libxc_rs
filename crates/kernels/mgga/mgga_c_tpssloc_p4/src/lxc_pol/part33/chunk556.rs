//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 556/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk556<F: Float>(t17: F, t5168: F, t1408: F, t3704: F, t1649: F, t3711: F, t1804: F, t3726: F, t131: F, t3732: F, t205: F, t1799: F, t213: F) -> (F, F, F, F, F, F) {
    let t5169 = t17 * t5168;
    let t5170 = t3704 * t1408;
    let t5178 = t3711 * t1649;
    let t5192 = t3726 * t1804;
    let t5194 = t3732 * t131;
    let t5195 = t205 * t5194;
    let t5196 = t213 * t1799;
    (t5169, t5170, t5178, t5192, t5195, t5196)
}
