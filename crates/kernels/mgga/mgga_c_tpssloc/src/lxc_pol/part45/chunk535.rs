//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 535/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk535<F: Float>(t344: F, t976: F, t381: F, t225: F, t387: F, t884: F, t1922: F, t986: F, t1049: F, t345: F, t340: F, t1054: F, t1065: F, t1945: F, t990: F, t131: F, t6679: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6688 = t976 * t344;
    let t6689 = t6688 * t381;
    let t6690 = t225 * t387;
    let t6691 = t6690 * t884;
    let t6692 = t6689 * t6691;
    let t6695 = t986 * t1922;
    let t6698 = t1049 * t225;
    let t6699 = t6698 * t387;
    let t6700 = t345 * t6699;
    let t6703 = t340 * t344;
    let t6704 = t6703 * t381;
    let t6705 = t225 * t1054;
    let t6706 = t6705 * t1065;
    let t6707 = t6704 * t6706;
    let t6710 = t990 * t1945;
    let t6712 = t6679 * t131;
    (t6688, t6689, t6690, t6691, t6692, t6695, t6699, t6700, t6703, t6704, t6705, t6706, t6707, t6710, t6712)
}
