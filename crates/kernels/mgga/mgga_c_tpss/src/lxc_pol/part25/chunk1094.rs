//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1094/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1094<F: Float>(t15088: F, t3931: F, t14920: F, t3933: F, t1465: F, t3754: F, t2741: F, t1407: F, t948: F, t11575: F, t4830: F, t949: F) -> (F, F, F, F, F) {
    let t15089 = t3931 * t15088;
    let t15092 = t14920 * t3933;
    let t15093 = t3931 * t15092;
    let t15096 = t1465 * t3754;
    let t15097 = t2741 * t15096;
    let t15100 = t1407 * t948;
    let t15101 = t11575 * t15100;
    let t15102 = t2741 * t15101;
    let t15107 = t4830 * t949;
    (t15089, t15093, t15097, t15102, t15107)
}
