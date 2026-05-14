//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1076/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1076<F: Float>(t15074: F, t15075: F, t345: F, t242: F, t947: F, t2724: F, t3949: F, t3932: F, t3931: F, t8561: F, t948: F, t14920: F, t3933: F, t1465: F, t3754: F, t2741: F) -> (F, F, F, F, F, F) {
    let t15076 = t15074 + t15075;
    let t15077 = t15076 * t345;
    let t15079 = t242 * t947 * t15077;
    let t15082 = t2724 * t3949;
    let t15083 = t3932 * t15082;
    let t15084 = t3931 * t15083;
    let t15087 = t8561 * t948;
    let t15088 = t14920 * t15087;
    let t15089 = t3931 * t15088;
    let t15092 = t14920 * t3933;
    let t15093 = t3931 * t15092;
    let t15096 = t1465 * t3754;
    let t15097 = t2741 * t15096;
    (t15076, t15079, t15084, t15089, t15093, t15097)
}
