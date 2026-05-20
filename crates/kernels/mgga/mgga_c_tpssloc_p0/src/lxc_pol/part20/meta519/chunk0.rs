//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2047/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2047<F: Float>(t2225: F, t3824: F, t12129: F, t588: F, t39035: F, t522: F, t39031: F, t1285: F, t9216: F, t9218: F, t16: F, t185: F, t520: F) -> (F, F, F, F, F, F, F) {
    let t39595 = F::new(120.0) * t2225 * t3824;
    let t39601 = t588 * t12129;
    let t39605 = t39035 * t522;
    let t39607 = t39031 * t522;
    let t39609 = t9216 * t1285;
    let t39611 = t9218 * t1285;
    let t39615 = F::new(24.0) * t16 * t520 * t185;
    (t39595, t39601, t39605, t39607, t39609, t39611, t39615)
}
