//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1116/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1116<F: Float>(t25: F, t28: F, t19541: F, t758: F, t12061: F, t6305: F, t3664: F, t5397: F, t16557: F, t2219: F, t5134: F, t514: F, t606: F, t12072: F, t6312: F, t3672: F, t5966: F, t1081: F, t18196: F, t5142: F, t517: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t19542 = t19541 * t758;
    let t19543 = 0.18311447306006545054e-3 * t19542;
    let t19547 = t12061 * t6305;
    let t19552 = t3664 * t5397;
    let t19558 = piecewise3(t26, 0.0, -8.0 / 27.0 * t19547 * t606 + 16.0 / 9.0 * t5134 * t2219 + 4.0 / 9.0 * t19552 * t606 + 4.0 / 3.0 * t514 * t16557);
    let t19559 = t12072 * t6312;
    let t19564 = t3672 * t5966;
    let t19570 = piecewise3(t29, 0.0, -8.0 / 27.0 * t19559 * t1081 - 16.0 / 9.0 * t5142 * t2219 + 4.0 / 9.0 * t19564 * t1081 + 4.0 / 3.0 * t517 * t18196);
    (t19543, t19558, t19570)
}
