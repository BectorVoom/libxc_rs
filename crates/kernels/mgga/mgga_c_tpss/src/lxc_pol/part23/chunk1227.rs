//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1227/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1227<F: Float>(t20827: F, t20852: F, t219: F, t6510: F, t1705: F, t4293: F, t935: F, t5570: F, t6513: F, t6030: F, t1148: F, t6516: F, t19123: F, t1586: F, t6032: F, t6034: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20853 = t20827 + t20852;
    let t20854 = param_beta * t20853;
    let t20856 = t6510 * t219;
    let t20862 = t1705 * t4293;
    let t20863 = t20862 * t935;
    let t20865 = t6513 * t5570;
    let t20868 = t6513 * t6030;
    let t20873 = t6516 * t1148;
    let t20874 = t19123 * t20873;
    let t20877 = t6032 * t1586;
    let t20878 = t20877 * t6034;
    (t20853, t20854, t20856, t20862, t20863, t20865, t20868, t20873, t20874, t20877, t20878)
}
