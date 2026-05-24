//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 775/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk775<F: Float>(t1098: F, t4216: F, t3032: F, t926: F, t4047: F, t1100: F, t4052: F, t1101: F, t3431: F, t1561: F, t461: F) -> (F, F, F, F, F, F, F, F) {
    let t4217 = t1098 * t4216;
    let t4219 = t926 * t3032;
    let t4220 = t4219 * t4047;
    let t4223 = t926 * t1100;
    let t4224 = t4223 * t4052;
    let t4227 = t1101 * t3431;
    let t4228 = t926 * t4227;
    let t4231 = t461 * t1561;
    (t4217, t4219, t4220, t4223, t4224, t4227, t4228, t4231)
}
