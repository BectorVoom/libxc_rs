//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1254/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1254<F: Float>(t3127: F, t42340: F, t42341: F, t23508: F, t3131: F, t381: F, t42422: F, t3199: F, t42741: F, t1057: F, t42754: F, t10474: F, t10482: F, t11060: F, t3120: F, t11045: F, t42332: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43515 = t42340 * t42341 * t3127;
    let t43516 = t23508 * t3131;
    let t43525 = t381 * t42422;
    let t43536 = t42741 * t3199;
    let t43542 = t42754 * t1057;
    let t43553 = t42340 * t42341 * t10474;
    let t43554 = t23508 * t10482;
    let t43558 = t11060 * t3120;
    let t43562 = t42332 * t11045;
    (t43515, t43516, t43525, t43536, t43542, t43553, t43554, t43558, t43562)
}
