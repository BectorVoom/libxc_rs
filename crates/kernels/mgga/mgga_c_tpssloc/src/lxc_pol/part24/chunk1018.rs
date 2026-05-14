//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1018/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1018<F: Float>(t15904: F, t8643: F, t22574: F, t3701: F, t3914: F, t2019: F, t1983: F, t6996: F, t6999: F, t1390: F, t3719: F, t6878: F, t1266: F, t1393: F, t1869: F, t1976: F, t1980: F, t22460: F, t22461: F, t22467: F, t22482: F, t22483: F, t22559: F, t22563: F, t2314: F, t2320: F, t2323: F, t3652: F, t3929: F, t510: F, t650: F, t6515: F, t6517: F, t652: F, t6539: F, t672: F, t6862: F, t6872: F) -> (F, F, F, F, F, F, F) {
    let t22575 = t8643 * t15904;
    let t22577 = 6.0 * t22574 * t22575;
    let t22578 = t3701 * t3914;
    let t22579 = t2019 * t22578;
    let t22580 = t1983 * t22579;
    let t22581 = t6996 * t6999;
    let t22583 = 2.0 * t1983 * t22581;
    let t22584 = t1390 * t3719;
    let t22585 = t6878 * t22584;
    let t22587 = 3.0 * t1983 * t22585;
    let t22588 = -2.0 * t1266 * t6515 + 2.0 * t1393 * t6872 - t1869 * t3652 - 2.0 * t1976 * t2320 + t1980 * t3929 - 4.0 * t22461 * t672 - 2.0 * t22483 * t652 - t22559 * t510 - 4.0 * t2314 * t6539 - 4.0 * t2323 * t6517 - 2.0 * t650 * t6862 - t22460 - t22467 - t22482 - t22563 - t22577 - t22580 - t22583 + t22587;
    (t22575, t22578, t22579, t22581, t22584, t22585, t22588)
}
