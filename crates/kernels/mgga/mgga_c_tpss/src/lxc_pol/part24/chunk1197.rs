//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1197/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1197<F: Float>(t1482: F, t5623: F, t5632: F, t1726: F, t4016: F, t1464: F, t5640: F, t985: F, t990: F, t18172: F, t3997: F, t18178: F, t6179: F, t342: F, t345: F, t3949: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19918 = t5623 * t1482;
    let t19919 = t5632 * t19918;
    let t19922 = t1726 * t4016;
    let t19923 = t5632 * t19922;
    let t19927 = t5640 * t1464;
    let t19928 = t985 * t990;
    let t19929 = t19927 * t19928;
    let t19932 = t18172 * t1464;
    let t19933 = t19932 * t3997;
    let t19936 = t18178 * t6179;
    let t19939 = t3949 * t342 * t345;
    (t19919, t19923, t19927, t19928, t19929, t19932, t19933, t19936, t19939)
}
