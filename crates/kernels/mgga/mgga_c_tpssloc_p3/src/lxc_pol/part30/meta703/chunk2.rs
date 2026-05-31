//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2287/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2287<F: Float>(t40: F, t5842: F, t1933: F, t23479: F, t17701: F, t17877: F, t18021: F, t1937: F, t1941: F, t23419: F, t28525: F, t28582: F, t378: F, t4579: F, t6722: F, t83117: F, t83215: F, t88422: F, t88425: F, t88428: F, t88440: F, t88453: F, t88513: F) -> (F, F) {
    let t99645 = t40 * t5842;
    let t99647 = t1933 * t99645 * t23479;
    let t99654 = -t88422 - t88425 - t88428 + t88513 * t4579 / F::cast_from(1152.0_f64) + t88440 + t88453 - t83215 * t17701 / F::cast_from(2304.0_f64) + t23419 * t18021 / F::cast_from(2304.0_f64) + t17877 * t1941 * t378 / F::cast_from(1536.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t99647 - F::cast_from(0.80745512188280781712e-3_f64) * t6722 * t28525 * t1937 - F::cast_from(0.10093189023535097714e-3_f64) * t83117 * t28582;
    (t99645, t99654)
}
