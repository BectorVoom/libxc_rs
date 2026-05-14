//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1153/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1153<F: Float>(t18403: F, t485: F, t626: F, t1163: F, t5531: F, t1688: F, t7798: F, t10456: F, t2056: F, t13146: F, t4347: F, t1165: F, t1695: F, t17942: F, t510: F, t517: F, t5543: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18404 = t485 * t18403;
    let t18406 = 2.0 * t626 * t18404;
    let t18409 = t1163 * t5531;
    let t18411 = 4.0 * t626 * t18409;
    let t18419 = 2.0 * t7798 * t1688;
    let t18421 = 4.0 * t10456 * t1688;
    let t18423 = 4.0 * t2056 * t5531;
    let t18425 = 2.0 * t13146 * t1688;
    let t18427 = 4.0 * t4347 * t5531;
    let t18429 = 2.0 * t1165 * t18403;
    let t18434 = t17942 * t510 * t1695;
    let t18435 = 35.0 / 432.0 * t18434;
    let t18436 = t5543 * t517;
    (t18404, t18406, t18409, t18411, t18419, t18421, t18423, t18425, t18427, t18429, t18435, t18436)
}
