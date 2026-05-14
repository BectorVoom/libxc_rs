//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1323/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1323<F: Float>(t70339: F, t70365: F, t70399: F, t70432: F, t21390: F, t940: F, t21391: F, t219: F, t1464: F, t19949: F, t4988: F, t5640: F, t15117: F, t15140: F, t15199: F, t1705: F, t1726: F, t1733: F, t18133: F, t18156: F, t18171: F, t18172: F, t19901: F, t19923: F, t19928: F, t19939: F, t19953: F, t21423: F, t21435: F, t373: F, t3997: F, t4008: F, t5018: F, t5626: F, t5631: F, t5632: F, t5639: F, t5642: F, t61292: F, t61476: F, t6172: F, t6179: F, t64573: F, t64694: F, t64714: F, t935: F, t991: F) -> (F, F, F) {
    let t70434 = t70339 + t70365 + t70399 + t70432;
    let t70437 = t940 * t21390;
    let t70453 = t21391 * t219;
    let t70462 = t19949 * t1464;
    let t70466 = t5640 * t4988;
    let t70486 = 4.0 * t19901 * t19923 + param_beta * t70434 * t373 - t5639 * t70437 * t5642 + 2.0 * t18133 * t5018 - 2.0 * t6172 * t19953 + 2.0 * t5626 * t15140 + 2.0 * t5631 * t5632 * t1726 * t15199 - 2.0 * t5639 * t19949 * t19939 - t70453 * t991 + 4.0 * t18156 * t64694 * t6179 - 4.0 * t18171 * t64714 * t1464 * t3997 + 2.0 * t18171 * t70462 * t4008 + 2.0 * t18156 * t70466 * t19928 - 2.0 * t18171 * t18172 * t4988 * t3997 + t18171 * t70466 * t4008 - t1705 * t15117 * t935 * t1733 + t61476 * t21435 - 2.0 * t5639 * t64573 * t6179 - 2.0 * t18171 * t61292 * t21423;
    (t70434, t70462, t70486)
}
