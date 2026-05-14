//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1071/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1071<F: Float>(t28: F, t1081: F, t18196: F, t19559: F, t19564: F, t2219: F, t5142: F, t517: F, t157: F, t19558: F, t184: F, t17: F, t6320: F, t750: F, t1388: F, t1799: F, t15877: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t19570 = piecewise3(t29, 0.0, -8.0 / 27.0 * t19559 * t1081 - 16.0 / 9.0 * t5142 * t2219 + 4.0 / 9.0 * t19564 * t1081 + 4.0 / 3.0 * t517 * t18196);
    let t19572 = (t19558 + t19570) * t157;
    let t19573 = t19572 * t184;
    let t19574 = t17 * t19573;
    let t19575 = t6320 * t750;
    let t19576 = t17 * t19575;
    let t19577 = t1799 * t1388;
    let t19581 = 16.0 * t15877;
    (t19572, t19574, t19576, t19577, t19581)
}
