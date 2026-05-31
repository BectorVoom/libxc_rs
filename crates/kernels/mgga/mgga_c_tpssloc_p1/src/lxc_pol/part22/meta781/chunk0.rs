//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2671/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2671<F: Float>(t39611: F, t39636: F, t57211: F, t17: F, t184: F, t74011: F, t54451: F, t20396: F, t750: F, t39845: F, t39615: F, t39620: F, t39655: F, t39658: F, t39844: F, t54439: F, t54447: F, t54453: F, t54457: F) -> (F, F, F, F, F, F, F, F) {
    let t74489 = F::cast_from(120.0_f64) * t39611;
    let t74490 = F::cast_from(24.0_f64) * t39636;
    let t74491 = F::cast_from(0.73245789224026180216e-3_f64) * t57211;
    let t74493 = t17 * t74011 * t184;
    let t74494 = F::cast_from(0.31168546390226634765e3_f64) * t54451;
    let t74496 = t17 * t20396 * t750;
    let t74497 = F::cast_from(60.0_f64) * t39845;
    let t74498 = -t54439 - t74489 - t39615 + t39620 + t54447 - t74490 + t74491 + t74493 + t74494 + t74496 - t39655 - t39658 + t54453 + t39844 + t74497 + t54457;
    (t74489, t74490, t74491, t74493, t74494, t74496, t74497, t74498)
}
