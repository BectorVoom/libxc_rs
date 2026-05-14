//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1115/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1115<F: Float>(t157: F, t19558: F, t19570: F, t184: F, t17: F, t6320: F, t750: F, t1388: F, t1799: F, t15877: F, t11979: F, t15890: F, t15895: F, t588: F, t6328: F, t592: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19572 = (t19558 + t19570) * t157;
    let t19573 = t19572 * t184;
    let t19574 = t17 * t19573;
    let t19575 = t6320 * t750;
    let t19576 = t17 * t19575;
    let t19577 = t1799 * t1388;
    let t19581 = 16.0 * t15877;
    let t19588 = 32.0 * t11979;
    let t19589 = 0.34631718211362927517e2 * t15890;
    let t19590 = 0.11696447245269292414e1 * t15895;
    let t19591 = t588 * t6328;
    let t19592 = 4.0 * t19591;
    let t19593 = t592 * t6328;
    (t19572, t19574, t19576, t19577, t19581, t19588, t19589, t19590, t19592, t19593)
}
