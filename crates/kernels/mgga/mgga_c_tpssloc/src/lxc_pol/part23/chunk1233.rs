//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1233/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1233<F: Float>(t136: F, t2826: F, t76597: F, t76593: F, t41880: F, t76572: F, t76576: F, t908: F, t76589: F, t10304: F, t76581: F, t76585: F, t68500: F, t68502: F, t68504: F, t68506: F) -> (F, F, F, F, F, F, F, F) {
    let t76877 = t136 * t2826 * t76597;
    let t76880 = t136 * t2826 * t76593;
    let t76887 = t136 * t41880 * t76572;
    let t76890 = t136 * t908 * t76576;
    let t76893 = t136 * t2826 * t76589;
    let t76896 = t136 * t10304 * t76581;
    let t76899 = t136 * t10304 * t76585;
    let t76901 = t76877 / 6.0 - 2.0 * t76880 - 16.0 / 81.0 * t68500 - 4.0 / 9.0 * t68502 - 8.0 / 3.0 * t68504 + 8.0 / 9.0 * t68506 + 14.0 / 81.0 * t76887 + t76890 / 6.0 + 2.0 * t76893 - 8.0 / 9.0 * t76896 + 4.0 / 9.0 * t76899;
    (t76877, t76880, t76887, t76890, t76893, t76896, t76899, t76901)
}
