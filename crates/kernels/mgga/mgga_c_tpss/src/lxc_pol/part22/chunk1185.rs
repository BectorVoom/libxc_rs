//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1185/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1185<F: Float>(t10514: F, t17930: F, t580: F, t750: F, t2133: F, t30: F, t159: F, t2138: F, t1695: F, t212: F, t223: F, t5543: F) -> (F, F, F, F, F, F) {
    let t17931 = t17930 * t10514;
    let t17934 = t580 * t750;
    let t17938 = t30 * t2133;
    let t17942 = t2138 * t159;
    let t17944 = t17942 * t212 * t1695;
    let t17946 = t5543 * t223;
    (t17931, t17934, t17938, t17942, t17944, t17946)
}
