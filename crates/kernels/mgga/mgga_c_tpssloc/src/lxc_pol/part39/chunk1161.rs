//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1161/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1161<F: Float>(t4072: F, t88: F, t1453: F, t666: F, t89: F, t2331: F, t626: F) -> (F, F, F, F) {
    let t26117 = t88 * t4072;
    let t26129 = t1453 * t666;
    let t26179 = t89 * t4072;
    let t29895 = t626 * t2331;
    (t26117, t26129, t26179, t29895)
}
