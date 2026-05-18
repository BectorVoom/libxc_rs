//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1262/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1262<F: Float>(t2022: F, t6483: F, t1864: F, t5389: F, t1863: F, t12571: F, t1410: F, t26012: F, t7441: F, t1437: F, t7445: F, t5445: F) -> (F, F, F, F, F, F) {
    let t96348 = t2022 * t6483;
    let t96425 = t1864 * t5389;
    let t96426 = t1863 * t96425;
    let t96443 = t12571 * t1410;
    let t96454 = t7441 * t26012;
    let t96461 = t7445 * t1437;
    let t96462 = t1863 * t96461;
    let t96469 = t1864 * t5445;
    (t96348, t96426, t96443, t96454, t96462, t96469)
}
