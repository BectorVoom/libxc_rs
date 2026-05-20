//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2098/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2098<F: Float>(t40904: F, t816: F, t2681: F, t9674: F, t812: F, t835: F, t9972: F, t9978: F, t9667: F, t9983: F, t2617: F, t9666: F) -> (F, F, F, F, F) {
    let t41399 = t40904 * t816;
    let t41404 = t9674 * t2681;
    let t41414 = t812 * t9972 * t835;
    let t41415 = t41414 * t9978;
    let t41417 = t9667 * t9983;
    let t41424 = t2617 * t9666;
    (t41399, t41404, t41415, t41417, t41424)
}
