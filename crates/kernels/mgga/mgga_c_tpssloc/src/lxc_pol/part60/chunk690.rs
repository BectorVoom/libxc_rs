//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 690/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk690<F: Float>(t6686: F, t8020: F, t1751: F, t7284: F, t24574: F, t8067: F, t477: F, t1419: F, t6794: F, t131: F, t467: F, t225: F, t8034: F, t7327: F, t24826: F, t8074: F) -> (F, F, F, F, F, F, F, F) {
    let t27406 = t8020 * t6686;
    let t27426 = t7284 * t1751;
    let t27451 = t24574 * t8067;
    let t27460 = t477 * t1751;
    let t27505 = t1419 * t6794;
    let t27506 = t27505 * t131;
    let t27507 = t27506 * t467;
    let t27516 = t8034 * t225;
    let t27536 = t8034 * t7327;
    let t27556 = t24826 * t8074;
    (t27406, t27426, t27451, t27460, t27507, t27516, t27536, t27556)
}
