//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1241/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1241<F: Float>(t22020: F, t22036: F, t1705: F, t5270: F, t935: F, t1880: F, t5275: F, t19123: F, t20877: F, t6521: F, t1586: F, t6509: F, t6025: F, t5294: F, t2785: F, t5248: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22037 = t22020 + t22036;
    let t22038 = param_beta * t22037;
    let t22045 = t1705 * t5270;
    let t22046 = t22045 * t935;
    let t22054 = t1880 * t5275;
    let t22055 = t19123 * t22054;
    let t22058 = t20877 * t6521;
    let t22061 = t6509 * t1586;
    let t22062 = t6025 * t22061;
    let t22065 = t1880 * t5294;
    let t22066 = t6025 * t22065;
    let t22069 = t5248 * t2785;
    (t22037, t22038, t22045, t22046, t22054, t22055, t22058, t22061, t22062, t22065, t22066, t22069)
}
