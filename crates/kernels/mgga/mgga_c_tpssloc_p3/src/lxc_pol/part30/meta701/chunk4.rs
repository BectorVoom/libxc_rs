//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2269/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2269<F: Float>(t97989: F, t98039: F, t98090: F, t99067: F, t1634: F, t607: F, t1065: F, t5392: F, t17686: F, t1927: F, t23327: F, t23329: F, t25424: F, t25429: F, t25430: F, t25442: F, t25738: F, t25815: F, t28701: F, t28702: F, t4337: F, t7553: F, t82342: F, t82402: F, t82417: F, t88004: F, t88050: F, t88069: F, t88075: F, t88083: F, t88089: F, t88112: F) -> (F, F, F, F) {
    let t99069 = t97989 + t98039 + t98090 + t99067;
    let t99070 = t1634 * t607;
    let t99099 = t5392 * t1065;
    let t99104 = -F::cast_from(0.73108180748810063845e-2_f64) * t25429 * t88112 * t4337 * t99070 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88004 * t7553 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88089 * t7553 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88050 * t25815 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t28702 - F::cast_from(0.3289868133696452873e-1_f64) * t1927 * t25442 * t25738 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t88050 * t25424 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t82417 * t28701 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23329 * t25430 * t17686 - t88069 - t88075 - t88083 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t82342 * t99099;
    (t99069, t99070, t99099, t99104)
}
