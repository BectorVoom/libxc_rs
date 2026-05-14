//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1221/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1221<F: Float>(t2157: F, t5831: F, t1395: F, t18770: F, t10841: F, t1378: F, t1707: F, t1708: F, t17993: F, t18000: F, t18006: F, t18009: F, t1809: F, t18784: F, t18800: F, t19736: F, t19767: F, t19769: F, t20446: F, t20466: F, t20470: F, t20482: F, t20488: F, t20503: F, t226: F, t228: F, t2364: F, t2407: F, t253: F, t44584: F, t44610: F, t5571: F, t5577: F, t5834: F, t61195: F, t61222: F, t61226: F, t6135: F, t6337: F, t6342: F, t63893: F, t64008: F, t64050: F, t64198: F, t66441: F, t782: F, t818: F) -> (F,) {
    let t66469 = t2157 * t5831;
    let t66480 = t18770 * t1395;
    let t66494 = 24.0 * t5571 * t61195 * t6342 * t2407 - 2.0 * t19736 * t18784 - t1707 * t1708 * t228 * t66441 + param_beta * t66441 * t253 - 2.0 * t18006 * t18770 * t64008 + 2.0 * t17993 * t20503 + 2.0 * t5571 * t5577 * t20446 * t782 * t226 + 2.0 * t5834 * t10841 - t64050 * t1809 + 8.0 * t18006 * t20482 * t1378 * t63893 + 2.0 * t19767 * t18770 * t44610 - 4.0 * t61222 * t20466 - 4.0 * t19767 * t66469 * t19769 - 4.0 * t19767 * t20482 * t44584 - 4.0 * t18006 * t18770 * t64198 - t6135 * t18800 + 12.0 * t61226 * t66480 * t18009 - 12.0 * t5571 * t18000 * t20470 * t818 + t5571 * t5577 * t6337 * t2364 * t226 + 2.0 * t17993 * t20488;
    (t66494,)
}
