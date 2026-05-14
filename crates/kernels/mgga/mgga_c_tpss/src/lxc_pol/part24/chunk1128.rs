//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1128/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1128<F: Float>(t159: F, t2138: F, t1695: F, t212: F, t223: F, t5543: F, t764: F, t1693: F, t238: F, t2157: F, t64: F, t234: F, t339: F, t5550: F, t789: F) -> (F, F, F, F, F, F, F, F) {
    let t17942 = t2138 * t159;
    let t17944 = t17942 * t212 * t1695;
    let t17945 = 35.0 / 432.0 * t17944;
    let t17946 = t5543 * t223;
    let t17947 = t17946 * t764;
    let t17949 = t1693 * t238;
    let t17954 = t2157 * t64;
    let t17956 = t339 * t17954 * t234;
    let t17960 = t339 * t5550 * t789;
    (t17942, t17945, t17946, t17947, t17949, t17954, t17956, t17960)
}
