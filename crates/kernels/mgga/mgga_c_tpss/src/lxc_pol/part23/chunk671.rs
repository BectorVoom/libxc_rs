//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 671/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk671<F: Float>(t3053: F, t466: F, t2785: F, t3054: F, t1113: F, t1133: F, t3073: F, t1107: F, t2712: F, t2711: F) -> (F, F, F, F, F) {
    let t3125 = t466 * t3053;
    let t3126 = t2785 * t3054;
    let t3130 = t1133 * t1113;
    let t3134 = t466 * t3073;
    let t3137 = t2712 * t1107;
    let t3138 = t2711 * t3137;
    (t3125, t3126, t3130, t3134, t3138)
}
