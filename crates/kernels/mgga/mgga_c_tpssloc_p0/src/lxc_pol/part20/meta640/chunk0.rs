//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2347/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2347<F: Float>(t10304: F, t136: F, t47742: F, t47701: F, t908: F, t2826: F, t47693: F, t47720: F, t47697: F, t41880: F, t47679: F, t133: F, t135: F, t241: F) -> (F, F, F, F, F, F, F) {
    let t48122 = t136 * t10304 * t47742;
    let t48125 = t136 * t908 * t47701;
    let t48128 = t136 * t2826 * t47693;
    let t48131 = t136 * t10304 * t47720;
    let t48134 = t136 * t2826 * t47697;
    let t48137 = t136 * t41880 * t47679;
    let t48140 = t133 * t135 * t241;
    (t48122, t48125, t48128, t48131, t48134, t48137, t48140)
}
