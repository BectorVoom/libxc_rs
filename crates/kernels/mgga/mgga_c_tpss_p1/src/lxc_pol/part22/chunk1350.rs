//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1350/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1350<F: Float>(t1805: F, t8275: F, t1378: F, t17993: F, t18000: F, t18006: F, t18750: F, t18767: F, t18770: F, t18789: F, t18794: F, t19736: F, t19748: F, t19767: F, t20446: F, t20475: F, t20479: F, t20482: F, t20483: F, t226: F, t2407: F, t3664: F, t5571: F, t5572: F, t5577: F, t5831: F, t5843: F, t61183: F, t61222: F, t61226: F, t62671: F, t6337: F, t6343: F, t6348: F, t64034: F, t64039: F, t64042: F, t64122: F, t64135: F, t64164: F, t64168: F, t64183: F, t818: F) -> F {
    let t66559 = t8275 * t1805;
    let t66601 = -F::new(4.0) * t18006 * t62671 * t19748 - F::new(2.0) * t18006 * t18770 * t64122 - F::new(4.0) * t61222 * t20479 - F::new(6.0) * t19736 * t18767 + F::new(4.0) * t17993 * t20475 + F::new(6.0) * t19767 * t66559 * t64164 - F::new(6.0) * t19767 * t20482 * t64168 - F::new(4.0) * t64034 * t20483 + F::new(6.0) * t61226 * t18770 * t64042 + F::new(2.0) * t64135 * t5843 + F::new(4.0) * t5571 * t5572 * t20446 * t818 + t19767 * t18770 * t64039 - F::new(2.0) * t19767 * t20482 * t64183 + F::new(2.0) * t19736 * t18789 + t19736 * t18794 + t5571 * t5577 * t18750 * t1378 * t226 + F::new(2.0) * t5571 * t5577 * t5831 * t3664 * t226 + t61183 * t6348 + F::new(2.0) * t61183 * t6343 - F::new(6.0) * t5571 * t18000 * t6337 * t2407;
    t66601
}
