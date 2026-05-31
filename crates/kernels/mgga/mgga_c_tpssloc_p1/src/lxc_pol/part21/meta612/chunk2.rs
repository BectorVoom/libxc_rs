//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2382/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2382<F: Float>(t2225: F, t3824: F, t1287: F, t9214: F, t12129: F, t588: F, t39033: F, t522: F, t1285: F, t9216: F, t9218: F, t16: F, t185: F, t520: F) -> (F, F, F, F, F, F, F) {
    let t39595 = F::cast_from(120.0_f64) * t2225 * t3824;
    let t39596 = t9214 * t1287;
    let t39601 = t588 * t12129;
    let t39603 = t39033 * t522;
    let t39609 = t9216 * t1285;
    let t39611 = t9218 * t1285;
    let t39615 = F::cast_from(24.0_f64) * t16 * t520 * t185;
    (t39595, t39596, t39601, t39603, t39609, t39611, t39615)
}
