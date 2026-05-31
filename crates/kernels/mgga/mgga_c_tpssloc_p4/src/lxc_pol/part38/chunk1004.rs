//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1004/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1004<F: Float>(t1287: F, t2225: F, t3681: F, t750: F, t17: F, t1284: F, t2516: F, t521: F, t9861: F, t3826: F, t592: F, t1285: F) -> (F, F, F, F, F, F) {
    let t12123 = F::cast_from(60.0_f64) * t2225 * t1287;
    let t12126 = t3681 * t750;
    let t12127 = t17 * t12126;
    let t12129 = t1284 * t2516;
    let t12130 = t17 * t12129;
    let t12132 = t521 * t9861;
    let t12133 = t17 * t12132;
    let t12134 = t592 * t3826;
    let t12136 = t2225 * t1285;
    (t12123, t12127, t12130, t12133, t12134, t12136)
}
