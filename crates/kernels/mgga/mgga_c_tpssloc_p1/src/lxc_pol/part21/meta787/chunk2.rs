//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2740/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2740<F: Float>(t12470: F, t193: F, t3924: F, t40224: F, t40230: F, t56486: F, t57226: F, t57228: F, t57230: F, t57231: F, t57232: F, t57233: F, t57236: F, t57237: F, t6330: F) -> F {
    let t57822 = F::new(6.0) * t12470 * t193 * t6330 + F::new(12.0) * t193 * t3924 * t56486 + t40224 - t40230 - t57226 + t57228 - t57230 - t57231 + t57232 + t57233 + t57236 + t57237;
    t57822
}
