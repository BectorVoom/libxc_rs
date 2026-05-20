//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 634/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk634<F: Float>(t2244: F, t2250: F, t2291: F, t2298: F, t634: F, t638: F, t72: F) -> F {
    let t2303 = F::new(28.0) / F::new(9.0) * t2291 * t2244 - F::new(4.0) / F::new(3.0) * t634 * t2250 + F::new(28.0) / F::new(9.0) * t2298 * t2244 + F::new(4.0) / F::new(3.0) * t638 * t2250;
    let t2304 = t72 * t2303;
    t2304
}
