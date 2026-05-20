//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2552/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2552<F: Float>(t449: F, t51703: F, t51723: F, t11365: F, t1694: F, t3331: F, t4794: F, t1117: F, t14913: F, t3313: F, t3315: F, t11185: F, t14937: F) -> (F, F, F, F, F) {
    let t51725 = (t51703 + t51723) * t449;
    let t51727 = t11365 * t1694;
    let t51730 = t4794 * t3331;
    let t51736 = F::cast_from(0.48245938496077605201e2_f64) * t3313 * t14913 * t3315 * t1117;
    let t51738 = F::new(18.0) * t11185 * t14937;
    (t51725, t51727, t51730, t51736, t51738)
}
