//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2699/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2699<F: Float>(t12461: F, t20684: F, t20085: F, t39655: F, t39658: F, t39844: F, t5160: F, t5356: F, t54453: F, t74490: F, t74491: F, t74493: F, t74494: F, t74496: F, t74497: F) -> (F, F) {
    let t75240 = t20684 * t12461;
    let t75254 = F::cast_from(6.0_f64) * t20085 * t5160 * t5356 - t39655 - t39658 + t39844 + t54453 - t74490 + t74491 + t74493 + t74494 + t74496 + t74497;
    (t75240, t75254)
}
