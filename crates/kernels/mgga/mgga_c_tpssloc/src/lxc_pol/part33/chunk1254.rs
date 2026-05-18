//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1254/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1254<F: Float>(t7604: F, t82632: F, t1920: F, t2966: F, t7614: F, t7607: F, t23518: F, t7577: F, t7561: F, t7557: F, t11094: F, t7627: F) -> (F, F, F, F, F, F, F) {
    let t89366 = t82632 * t7604;
    let t89431 = t1920 * t2966 * t7614;
    let t89449 = t82632 * t7607;
    let t89473 = t7577 * t23518;
    let t89617 = t1920 * t2966 * t7561;
    let t89672 = t82632 * t7557;
    let t89702 = t7627 * t11094;
    (t89366, t89431, t89449, t89473, t89617, t89672, t89702)
}
