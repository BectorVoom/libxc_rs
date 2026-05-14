//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1141/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1141<F: Float>(t7709: F, t80766: F, t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t22573: F, t7684: F, t531: F, t7752: F, t2022: F, t6483: F, t1864: F, t5389: F, t1863: F) -> (F, F, F, F, F, F, F) {
    let t91400 = t80766 * t7709;
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91655 = t7684 * t22573;
    let t91675 = t531 * t7752;
    let t96348 = t2022 * t6483;
    let t96425 = t1864 * t5389;
    let t96426 = t1863 * t96425;
    (t91400, t91531, t91548, t91655, t91675, t96348, t96426)
}
