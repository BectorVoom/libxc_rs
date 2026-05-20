//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 781/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk781<F: Float>(t1268: F, t2314: F, t2363: F, t5113: F, t671: F, t9347: F, t9348: F, t9351: F, t9416: F, t195: F, t40: F, t2433: F, t607: F) -> (F, F, F) {
    let t9419 = F::new(2.0) * t1268 * t9416 + F::new(6.0) * t2314 * t2363 + F::new(6.0) * t2363 * t5113 + F::new(6.0) * t671 * t9348 + t9347 + F::new(6.0) * t9351;
    let t9427 = F::new(1.0) / t195 / t40;
    let t9430 = t2433 * t607;
    (t9419, t9427, t9430)
}
