//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1229/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1229<F: Float>(t18172: F, t21423: F, t19949: F, t6179: F, t342: F, t345: F, t4988: F, t5640: F, t21422: F, t1731: F, t21390: F, t347: F, t1483: F, t1730: F, t1733: F, t18156: F, t18171: F, t19892: F, t19901: F, t19904: F, t21391: F, t21399: F, t21408: F, t21411: F, t21415: F, t21419: F, t373: F, t5018: F, t5037: F, t5626: F, t5631: F, t5639: F, t6172: F, t6175: F, t6180: F, t6183: F) -> (F, F, F, F, F, F, F, F) {
    let t21424 = t18172 * t21423;
    let t21427 = t19949 * t6179;
    let t21431 = t4988 * t342 * t345;
    let t21432 = t5640 * t21431;
    let t21434 = t21422 * t345;
    let t21435 = t5640 * t21434;
    let t21438 = t1731 * t347 * t21390;
    let t21440 = -2.0 * t1483 * t19892 - t1730 * t21438 - t1733 * t21399 + 4.0 * t18156 * t21411 - 2.0 * t18171 * t21424 + t18171 * t21435 + 4.0 * t19901 * t6175 - 2.0 * t19904 * t6180 + t21391 * t373 - 6.0 * t21408 * t5631 + 4.0 * t21415 * t5631 + 2.0 * t21419 * t5631 - 2.0 * t21427 * t5639 - t21432 * t5639 + 2.0 * t5018 * t5626 - t5037 * t5626 - 2.0 * t6172 * t6183;
    (t21424, t21427, t21431, t21432, t21434, t21435, t21438, t21440)
}
