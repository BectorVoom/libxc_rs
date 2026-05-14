//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1228/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1228<F: Float>(t21390: F, t1705: F, t5012: F, t935: F, t1726: F, t5017: F, t18150: F, t19913: F, t6179: F, t1482: F, t6167: F, t5632: F, t5036: F, t2785: F, t4977: F, t2724: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21391 = param_beta * t21390;
    let t21398 = t1705 * t5012;
    let t21399 = t21398 * t935;
    let t21407 = t1726 * t5017;
    let t21408 = t18150 * t21407;
    let t21411 = t19913 * t6179;
    let t21414 = t6167 * t1482;
    let t21415 = t5632 * t21414;
    let t21418 = t1726 * t5036;
    let t21419 = t5632 * t21418;
    let t21422 = t4977 * t2785;
    let t21423 = t21422 * t2724;
    (t21391, t21398, t21399, t21407, t21408, t21411, t21414, t21415, t21418, t21419, t21422, t21423)
}
