//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1041/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1041<F: Float>(t6553: F, t6554: F, t81984: F, t9458: F, t23205: F, t82038: F, t23242: F, t81979: F, t10140: F, t25: F, t193: F, t10121: F, t22960: F, t46240: F, t1081: F, t2752: F) -> (F, F, F, F, F, F, F, F) {
    let t82282 = t81984 * t6553 * t6554 * t9458;
    let t82294 = t82038 * t23205;
    let t82296 = t81979 * t23242;
    let t82313 = t25 * t10140;
    let t82320 = t193 * t9458;
    let t82323 = t25 * t10121;
    let t82330 = t22960 * t46240;
    let t83555 = t2752 * t1081;
    (t82282, t82294, t82296, t82313, t82320, t82323, t82330, t83555)
}
