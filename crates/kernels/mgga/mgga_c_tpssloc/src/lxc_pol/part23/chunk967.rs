//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 967/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk967<F: Float>(t10996: F, t20234: F, t974: F, t1616: F, t5685: F, t3071: F, t5677: F, t10408: F, t1539: F, t5867: F, t21118: F, t248: F, t3062: F, t21238: F, t942: F, t951: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21561 = t10996 * t20234;
    let t21562 = t974 * t21561;
    let t21565 = t5685 * t1616;
    let t21566 = t3071 * t21565;
    let t21569 = t5677 * t1616;
    let t21570 = t10408 * t21569;
    let t21573 = t5867 * t1539;
    let t21574 = t3071 * t21573;
    let t21580 = t248 * t3062 * t21118;
    let t21589 = t942 * t21238 * t951;
    (t21561, t21562, t21565, t21566, t21569, t21570, t21573, t21574, t21580, t21589)
}
