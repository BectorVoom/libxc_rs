//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 898/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk898<F: Float>(t115925: F, t28831: F, t33363: F, t7756: F, t33623: F, t7685: F, t101138: F, t26161: F, t33221: F, t1983: F, t20085: F, t8640: F, t29377: F, t8643: F, t126022: F, t127539: F, t128555: F, t128562: F, t128564: F, t128567: F, t128571: F, t128573: F, t2096: F, t29222: F, t29252: F, t33133: F, t510: F, t6468: F, t7904: F, t8450: F, t8604: F) -> (F,) {
    let t128575 = 6.0 * t115925 * t28831;
    let t128577 = 2.0 * t33363 * t7756;
    let t128581 = 2.0 * t7685 * t33623;
    let t128584 = 4.0 * t26161 * t101138 * t33221;
    let t128588 = 2.0 * t1983 * t8640 * t20085;
    let t128592 = t1983 * t29377 * t8643;
    let t128593 = t126022 * t2096 - 2.0 * t128555 * t510 - t29222 * t8450 + 6.0 * t29252 * t8450 + 6.0 * t33133 * t7904 + t6468 * t8604 - t127539 + t128562 + t128564 + t128567 + t128571 - t128573 - t128575 - t128577 - t128581 + t128584 + t128588 - t128592;
    (t128593,)
}
