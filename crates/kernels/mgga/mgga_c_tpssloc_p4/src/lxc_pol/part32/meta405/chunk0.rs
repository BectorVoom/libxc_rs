//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1546/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1546<F: Float>(t10165: F, t18070: F, t225: F, t5915: F, t1049: F, t5872: F, t3201: F, t3188: F, t1057: F, t18028: F, t1615: F, t4657: F) -> (F, F, F, F, F, F) {
    let t18071 = t10165 * t18070;
    let t18074 = t5915 * t225;
    let t18080 = t1049 * t5872;
    let t18081 = t18080 * t3201;
    let t18083 = t18080 * t3188;
    let t18086 = t18028 * t1057;
    let t18088 = t4657 * t1615;
    (t18071, t18074, t18081, t18083, t18086, t18088)
}
