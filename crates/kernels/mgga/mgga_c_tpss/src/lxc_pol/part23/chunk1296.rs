//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1296/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1296<F: Float>(t114: F, t65457: F, t485: F, t626: F, t18544: F, t6275: F, t19614: F, t5706: F, t18290: F, t6243: F, t18547: F, t41839: F, t7029: F, t1760: F, t18533: F, t4525: F, t6246: F) -> (F, F, F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t65458 = piecewise3(t115, 0.0, t65457);
    let t65461 = 2.0 * t626 * t485 * t65458;
    let t65472 = t18544 * t6275;
    let t65474 = 6.0 * t5706 * t19614;
    let t65480 = 6.0 * t6243 * t18290;
    let t65483 = 3.0 * t18547 * t7029 * t41839;
    let t65485 = t1760 * t18533 * t4525;
    let t65487 = 3.0 * t18544 * t6246;
    (t65458, t65461, t65472, t65474, t65480, t65483, t65485, t65487)
}
