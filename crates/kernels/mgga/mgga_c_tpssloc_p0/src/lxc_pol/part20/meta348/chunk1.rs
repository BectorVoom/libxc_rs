//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1651/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1651<F: Float>(t12155: F, t12156: F, t1307: F, t1365: F, t3719: F, t12012: F, t1347: F, t12147: F, t1345: F, t1348: F, t3839: F, t3844: F, t3847: F, t5278: F, t546: F, t548: F) -> (F, F, F, F) {
    let t12157 = t12155 * t12156;
    let t12160 = t1365 * t1307;
    let t12161 = t12160 * t3719;
    let t12164 = t1347 * t12012;
    let t12167 = -t12147 * t548 + F::cast_from(60.0_f64) * t12157 * t546 - F::cast_from(36.0_f64) * t12161 * t5278 + F::cast_from(3.0_f64) * t12164 * t546 - F::cast_from(36.0_f64) * t1345 * t3844 + F::cast_from(9.0_f64) * t1345 * t3847 + F::cast_from(9.0_f64) * t1348 * t3839;
    (t12157, t12161, t12164, t12167)
}
