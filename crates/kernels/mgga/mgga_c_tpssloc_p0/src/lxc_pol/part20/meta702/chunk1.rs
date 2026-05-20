//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2672/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2672<F: Float>(t39615: F, t39639: F, t39655: F, t39658: F, t39844: F, t54442: F, t54443: F, t54444: F, t54445: F, t54446: F, t54447: F, t54448: F, t54449: F, t54450: F, t54452: F, t54453: F) -> F {
    let t54454 = -t54442 - t39615 + t54443 + t54444 - t54445 - t54446 - t54447 - t54448 + t39639 + t54449 - t54450 + t54452 - t39655 - t39658 - t54453 + t39844;
    t54454
}
