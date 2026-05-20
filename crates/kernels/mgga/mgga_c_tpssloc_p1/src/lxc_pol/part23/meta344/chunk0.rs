//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1130/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1130<F: Float>(t2225: F, t3824: F, t1287: F, t9214: F, t39033: F, t522: F, t39035: F, t39031: F, t16: F, t185: F, t520: F, t9212: F) -> (F, F, F, F, F, F, F) {
    let t39595 = F::new(120.0) * t2225 * t3824;
    let t39596 = t9214 * t1287;
    let t39597 = F::new(576.0) * t39596;
    let t39603 = t39033 * t522;
    let t39604 = F::new(1440.0) * t39603;
    let t39605 = t39035 * t522;
    let t39606 = F::new(1920.0) * t39605;
    let t39607 = t39031 * t522;
    let t39608 = F::new(384.0) * t39607;
    let t39615 = F::new(24.0) * t16 * t520 * t185;
    let t39634 = t9212 * t1287;
    (t39595, t39597, t39604, t39606, t39608, t39615, t39634)
}
