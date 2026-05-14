//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 670/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk670<F: Float>(t1013: F, t4056: F, t128: F, t2835: F, t2836: F, t4044: F, t4049: F, t4054: F, t408: F, t1023: F, t1505: F, t1044: F, t1519: F, t2857: F, t1042: F, t2862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4057 = t1013 * t4056;
    let t4058 = t128 * t4057;
    let t4060 = t2835 - 0.5936111111111111111e-2 * t2836 - 0.5936111111111111111e-2 * t4044 - 0.11872222222222222222e-1 * t4049 + 0.35616666666666666666e-1 * t4054 + 0.17808333333333333333e-1 * t4058;
    let t4062 = 0.621814e-1 * t4060 * t408;
    let t4063 = t1505 * t1023;
    let t4065 = 1.0 * t4063 * t1044;
    let t4067 = 1.0 * t2857 * t1519;
    let t4068 = t1519 * t1042;
    let t4070 = 2.0 * t2862 * t4068;
    (t4057, t4058, t4060, t4062, t4063, t4065, t4067, t4068, t4070)
}
