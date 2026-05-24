//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1343/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1343<F: Float>(t198: F, t205: F, t5848: F, t206: F, t6353: F, t2161: F, t6337: F, t768: F, t10764: F, t10837: F, t17991: F, t17993: F, t18000: F, t18006: F, t18009: F, t1805: F, t18753: F, t18770: F, t18771: F, t18797: F, t19736: F, t19762: F, t19767: F, t19781: F, t20463: F, t20474: F, t20482: F, t20494: F, t20506: F, t226: F, t2425: F, t3699: F, t5568: F, t5571: F, t5577: F, t5834: F, t62671: F, t6342: F, t6351: F, t64034: F, t64060: F, t64063: F, t64118: F, t64190: F, t64204: F, t818: F) -> (F, F, F, F) {
    let t66311 = t198 * t205 * t5848;
    let t66317 = t198 * t206 * t6353;
    let t66328 = t6337 * t2161;
    let t66362 = t768 * t6337;
    let t66379 = t5571 * t5577 * t66328 * t226 - F::new(6.0) * t5571 * t18000 * t6342 * t2425 + F::new(4.0) * t18753 * t3699 - F::new(2.0) * t18006 * t18770 * t64190 - F::new(4.0) * t64060 * t18771 + F::new(4.0) * t18006 * t20482 * t64063 + t19736 * t18797 + F::new(2.0) * t19767 * t62671 * t19781 - F::new(12.0) * t5571 * t18000 * t20474 * t818 + F::new(2.0) * t64034 * t20494 - F::new(4.0) * t18006 * t62671 * t19762 - F::new(4.0) * t18006 * t18770 * t64118 - F::new(4.0) * t18006 * t66362 * t18009 + t19767 * t18770 * t64204 + t5571 * t5577 * t1805 * t10764 * t226 - t17991 * t6351 + F::new(4.0) * t5834 * t10837 - F::new(12.0) * t17993 * t20463 - F::new(2.0) * t5568 * t20506;
    (t66311, t66317, t66328, t66379)
}
