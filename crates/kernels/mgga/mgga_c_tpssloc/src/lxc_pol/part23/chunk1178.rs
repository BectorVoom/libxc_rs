//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1178/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1178<F: Float>(t12461: F, t20684: F, t571: F, t6330: F, t20193: F, t604: F, t1409: F, t1426: F, t67: F, t20305: F, t626: F, t20308: F, t20343: F, t1858: F, t6470: F, t1851: F, t6483: F) -> (F, F, F, F, F, F, F, F, F) {
    let t75240 = t20684 * t12461;
    let t75256 = t6330 * t571;
    let t75284 = t20193 * t604;
    let t75361 = t1409 * t1426 * t67;
    let t75592 = t626 * t20305;
    let t75601 = t626 * t20308;
    let t75613 = t626 * t20343;
    let t75768 = t6470 * t1858;
    let t75774 = t1851 * t6483;
    (t75240, t75256, t75284, t75361, t75592, t75601, t75613, t75768, t75774)
}
