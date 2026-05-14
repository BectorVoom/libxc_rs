//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 687/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk687<F: Float>(t4251: F, t4292: F, t1579: F, t219: F, t1148: F, t1586: F, t3118: F, t1113: F, t3126: F, t1133: F, t1561: F, t4245: F, t466: F, t2785: F, t450: F, t1578: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4293 = t4251 + t4292;
    let t4294 = param_beta * t4293;
    let t4296 = t1579 * t219;
    let t4299 = t1586 * t1148;
    let t4300 = t3118 * t4299;
    let t4303 = t3126 * t1113;
    let t4307 = t1133 * t1561;
    let t4310 = t466 * t4245;
    let t4314 = t2785 * t1113 * t450;
    let t4317 = t1578 * t1113;
    (t4293, t4294, t4296, t4300, t4303, t4307, t4310, t4314, t4317)
}
