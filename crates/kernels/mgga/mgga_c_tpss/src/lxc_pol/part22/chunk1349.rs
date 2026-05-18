//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1349/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1349<F: Float>(t20447: F, t219: F, t10833: F, t10894: F, t10895: F, t1378: F, t1395: F, t1396: F, t17993: F, t18006: F, t18009: F, t18021: F, t1805: F, t18750: F, t18753: F, t18770: F, t18775: F, t18779: F, t19734: F, t19736: F, t20449: F, t20471: F, t20492: F, t20498: F, t2162: F, t2408: F, t2425: F, t2426: F, t3721: F, t3722: F, t5571: F, t5572: F, t5831: F, t5834: F, t5838: F, t5846: F, t62731: F, t6337: F, t64135: F, t66328: F, t819: F) -> F {
    let t66525 = t20447 * t219;
    let t66546 = F::new(2.0) * t17993 * t20492 - F::new(2.0) * t18753 * t3722 - t62731 * t1396 - F::new(4.0) * t18006 * t18770 * t1378 * t18009 - t5834 * t10895 - F::new(6.0) * t5834 * t10833 - F::new(2.0) * t5571 * t18021 * t66328 * t2162 + F::new(2.0) * t19736 * t18779 + F::new(2.0) * t5571 * t5572 * t18750 * t1395 - t20449 * t2426 + F::new(4.0) * t64135 * t5838 + F::new(4.0) * t5571 * t5572 * t5831 * t3721 - F::new(2.0) * t66525 * t819 + F::new(4.0) * t17993 * t20498 + F::new(2.0) * t20449 * t2408 + F::new(4.0) * t17993 * t20471 + F::new(2.0) * t5571 * t5572 * t1805 * t10894 + F::new(2.0) * t5571 * t5572 * t6337 * t2425 - F::new(2.0) * t19734 * t5846 + F::new(4.0) * t19736 * t18775;
    t66546
}
