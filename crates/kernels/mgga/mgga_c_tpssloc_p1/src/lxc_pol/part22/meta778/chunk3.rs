//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2666/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2666<F: Float>(t39309: F, t39312: F, t39316: F, t39320: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t54315: F, t54317: F, t74017: F, t74024: F, t74026: F, t74027: F, t74028: F) -> F {
    let t74467 = t74017 - t39309 + t39312 + t39316 + t39320 - t39324 + t74024 + t54315 + t54317 + t39327 + t74026 - t39338 + t74027 - t74028 + t39346 + t39349 + t39356;
    t74467
}
