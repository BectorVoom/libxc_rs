//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2060/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2060<F: Float>(t205: F, t40024: F, t12247: F, t551: F, t236: F, t3792: F, t12283: F, t12422: F, t12339: F, t3876: F, t10021: F, t1336: F, t1361: F) -> (F, F, F, F, F, F, F) {
    let t40025 = t205 * t40024;
    let t40041 = F::new(1.0) / t12247 / t551;
    let t40042 = t40041 * t236;
    let t40046 = t3792 * t3792;
    let t40052 = t12283 * t12422;
    let t40054 = t12339 * t3876;
    let t40059 = t1336 * t1361 * t10021;
    (t40025, t40041, t40042, t40046, t40052, t40054, t40059)
}
