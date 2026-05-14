//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1065/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1065<F: Float>(t31483: F, t31517: F, t113: F, t1874: F, t23938: F, t26977: F, t6525: F, t7042: F, t7217: F, t8643: F, t1983: F, t1976: F, t2036: F, t31294: F, t31296: F, t31298: F, t31302: F, t31305: F, t31306: F, t6862: F, t7040: F) -> (F, F, F) {
    let t31518 = t31483 + t31517;
    let t31519 = t113 * t31518;
    let t31521 = 2.0 * t23938 * t1874;
    let t31523 = 2.0 * t26977 * t1874;
    let t31525 = 2.0 * t7042 * t6525;
    let t31526 = t7217 * t8643;
    let t31527 = t1983 * t31526;
    let t31528 = -t1976 * t7040 - t2036 * t6862 + t31294 - t31296 - t31298 - t31302 + t31305 + t31306 - t31519 - t31521 - t31523 - t31525 - t31527;
    (t31518, t31526, t31528)
}
