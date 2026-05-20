//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2033/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2033<F: Float>(t1284: F, t17: F, t9861: F, t1287: F, t9212: F, t1285: F, t9218: F, t118: F, t142: F, t39283: F) -> (F, F, F, F, F) {
    let t39620 = t17 * t1284 * t9861;
    let t39634 = t9212 * t1287;
    let t39636 = t9212 * t1285;
    let t39655 = F::new(480.0) * t9218 * t1287;
    let t39658 = F::cast_from(0.11483599538271604938e-1_f64) * t118 * t39283 * t142;
    (t39620, t39634, t39636, t39655, t39658)
}
