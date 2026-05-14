//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 543/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk543<F: Float>(t249: F, t7503: F, t1512: F, t6614: F, t1516: F, t6621: F, t6580: F, t6587: F, t6603: F, t6618: F, t7494: F, t7498: F, t7501: F) -> (F, F, F, F) {
    let t7504 = t7503 * t249;
    let t7506 = t6614 * t1512;
    let t7508 = t6621 * t1516;
    let t7510 = -t6580 - t7494 / 48.0 - t6587 - 0.12111826828242117256e-2 * t7498 - t6603 - 0.20186378047070195427e-3 * t7501 + t7504 / 1536.0 - t7506 / 1536.0 - t6618 - t7508 / 384.0;
    (t7504, t7506, t7508, t7510)
}
