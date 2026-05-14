//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1058/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1058<F: Float>(t649: F, t8319: F, t510: F, t1266: F, t8320: F, t6504: F, t8307: F, t8513: F, t6999: F, t8489: F, t1983: F, t3701: F, t6995: F, t2019: F, t6880: F, t8450: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30991 = t649 * t8319;
    let t30993 = 2.0 * t30991 * t510;
    let t30995 = 2.0 * t8320 * t1266;
    let t31019 = t8513 * t8307 * t6504;
    let t31033 = t8489 * t6999;
    let t31034 = t1983 * t31033;
    let t31035 = t3701 * t6995;
    let t31036 = t2019 * t31035;
    let t31038 = 2.0 * t1983 * t31036;
    let t31039 = t8450 * t6880;
    (t30991, t30993, t30995, t31019, t31033, t31034, t31036, t31038, t31039)
}
