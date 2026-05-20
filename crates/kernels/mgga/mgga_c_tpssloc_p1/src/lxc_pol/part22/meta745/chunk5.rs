//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2478/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2478<F: Float>(t1041: F, t10949: F, t14211: F, t21487: F, t21538: F, t21562: F, t2960: F, t3130: F, t4582: F, t4588: F, t4596: F, t4600: F, t61736: F, t61739: F, t62091: F, t62137: F, t62148: F, t62150: F, t62152: F, t70458: F) -> F {
    let t70481 = F::new(5.0) / F::new(13824.0) * t1041 * t4582 * t4588 * t70458 + F::new(2.0) / F::new(27.0) * t2960 * t21538 - t2960 * t21562 / F::new(18.0) + t62137 / F::new(3456.0) - t62148 / F::new(2304.0) - t62150 / F::new(432.0) + t62152 / F::new(768.0) + t10949 * t21487 / F::new(512.0) + t3130 * t4582 * t62091 * t14211 / F::new(512.0) + t61736 * t4596 / F::new(512.0) - t61739 * t4600 / F::new(1024.0);
    t70481
}
