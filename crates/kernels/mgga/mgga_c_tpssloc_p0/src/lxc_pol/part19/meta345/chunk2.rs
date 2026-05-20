//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1236/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1236<F: Float>(t2686: F, t9674: F, t2697: F, t9618: F, t40904: F, t816: F, t2681: F, t2629: F, t9612: F, t812: F, t835: F, t9972: F) -> (F, F, F, F, F, F) {
    let t41395 = t9674 * t2686;
    let t41397 = t2697 * t9618;
    let t41399 = t40904 * t816;
    let t41404 = t9674 * t2681;
    let t41410 = t9612 * t2629;
    let t41414 = t812 * t9972 * t835;
    (t41395, t41397, t41399, t41404, t41410, t41414)
}
