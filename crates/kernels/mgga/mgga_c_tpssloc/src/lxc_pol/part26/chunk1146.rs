//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1146/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1146<F: Float>(t131: F, t2587: F, t81142: F, t1905: F, t9537: F, t23004: F, t23110: F, t23185: F, t22987: F, t25038: F, t25248: F, t2553: F, t23005: F, t6579: F, t2631: F, t852: F) -> (F, F, F, F, F, F) {
    let t81686 = t81142 * t2587 * t131;
    let t81688 = t81686 * t9537 * t1905;
    let t81689 = 0.13707783890401886971e-2 * t81688;
    let t81691 = t23185 * t23110 * t23004;
    let t81695 = t25038 * t25248 * t22987 * t2553;
    let t81697 = t6579 * t23005;
    let t81699 = t852 * t2631;
    (t81686, t81689, t81691, t81695, t81697, t81699)
}
