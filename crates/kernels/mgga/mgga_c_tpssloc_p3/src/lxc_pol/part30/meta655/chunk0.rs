//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2071/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2071<F: Float>(t7557: F, t82632: F, t25836: F, t3216: F, t11094: F, t7627: F, t28: F, t40772: F, t1649: F, t2752: F, t26012: F, t6505: F) -> (F, F, F, F, F, F) {
    let t89672 = t82632 * t7557;
    let t89698 = t25836 * t3216;
    let t89702 = t7627 * t11094;
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90087 = t6505 * t26012;
    (t89672, t89698, t89702, t89953, t89992, t90087)
}
