//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1240/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1240<F: Float>(t20329: F, t20363: F, t20395: F, t20646: F, t3: F, t1799: F, t645: F, t1338: F, t19040: F, t3537: F, t5953: F, t116: F, t6323: F) -> (F, F, F, F, F, F, F, F) {
    let t20648 = t20329 + t20363 + t20395 + t20646;
    let t20649 = t3 * t20648;
    let t20660 = param_d * t20648;
    let t20678 = t645 * t1799;
    let t20679 = t20678 * t1338;
    let t20682 = t19040 * t1338;
    let t20685 = t5953 * t3537;
    let t20690 = t116 * t6323;
    (t20648, t20649, t20660, t20678, t20679, t20682, t20685, t20690)
}
