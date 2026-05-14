//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1239/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1239<F: Float>(t21854: F, t508: F, t2157: F, t6337: F, t1378: F, t14297: F, t14372: F, t17993: F, t18000: F, t18006: F, t1805: F, t18753: F, t18770: F, t19736: F, t19748: F, t19767: F, t19769: F, t19781: F, t20446: F, t20475: F, t20479: F, t20482: F, t20498: F, t21299: F, t21608: F, t21650: F, t226: F, t3664: F, t4758: F, t4783: F, t4799: F, t4800: F, t52460: F, t5571: F, t5572: F, t5577: F, t5831: F, t5834: F, t5846: F, t64060: F, t66362: F, t66480: F, t70070: F, t818: F) -> (F, F) {
    let t71884 = t508 * t21854;
    let t71935 = t2157 * t6337;
    let t71970 = -t21299 * t5846 + 2.0 * t5834 * t14372 + 2.0 * t5571 * t5572 * t5831 * t4799 + 2.0 * t5571 * t5572 * t21608 * t818 + 4.0 * t19736 * t20475 - 4.0 * t18006 * t66480 * t19781 + 4.0 * t19736 * t20498 - 6.0 * t5571 * t18000 * t5831 * t4783 - 4.0 * t19767 * t71935 * t19769 - 2.0 * t19767 * t20482 * t52460 - 4.0 * t18006 * t66362 * t19748 - 2.0 * t18006 * t18770 * t70070 + t5571 * t5577 * t5831 * t4758 * t226 + t5571 * t5577 * t1805 * t14297 * t226 - t18753 * t4800 + 2.0 * t5571 * t5577 * t20446 * t1378 * t226 + 2.0 * t5571 * t5577 * t6337 * t3664 * t226 + t17993 * t21650 - 4.0 * t64060 * t20479;
    (t71884, t71970)
}
