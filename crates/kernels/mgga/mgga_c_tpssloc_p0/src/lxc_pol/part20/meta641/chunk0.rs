//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2348/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2348<F: Float>(t42444: F, t45971: F, t48140: F, t2770: F, t340: F, t43317: F, t136: F, t47746: F, t908: F, t2403: F, t4389: F, t4386: F) -> (F, F, F, F, F, F, F) {
    let t48142 = t48140 * t42444 * t45971;
    let t48143 = t340 * t2770;
    let t48145 = t48140 * t48143 * t45971;
    let t48148 = t48140 * t43317 * t45971;
    let t48153 = t136 * t908 * t47746;
    let t48155 = t2403 * t4389;
    let t48156 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t48155;
    let t48157 = t2403 * t4386;
    (t48142, t48145, t48148, t48153, t48155, t48156, t48157)
}
