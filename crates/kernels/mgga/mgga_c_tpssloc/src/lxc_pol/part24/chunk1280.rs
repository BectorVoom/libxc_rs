//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1280/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1280<F: Float>(t72: F, t79: F, t9342: F, t1865: F, t22523: F, t22531: F, t22537: F, t22546: F, t22554: F, t605: F, t6490: F, t6492: F, t6506: F, t6510: F, t83814: F, t83817: F, t83820: F, t83822: F, t83827: F, t83830: F, t83832: F, t83835: F, t83840: F) -> (F,) {
    let t83846 = t72 * t79 * t9342;
    let t83849 = -5.0 * t83814 * t6492 + t605 * t83817 * t83820 + t83822 * t1865 / 3.0 + t22537 * t6506 + t22537 * t6510 - 15.0 * t83827 * t22546 + 35.0 * t83830 * t83832 + t83835 * t1865 + 5.0 / 2.0 * t22523 * t22531 + 5.0 / 2.0 * t6490 * t83840 + 5.0 / 2.0 * t22554 * t22531 + 5.0 / 6.0 * t6490 * t83846;
    (t83849,)
}
