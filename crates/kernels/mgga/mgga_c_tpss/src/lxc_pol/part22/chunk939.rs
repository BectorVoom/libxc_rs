//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 939/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk939<F: Float>(t10894: F, t812: F, t10819: F, t10821: F, t10833: F, t10837: F, t10841: F, t1396: F, t2401: F, t2408: F, t2426: F, t253: F, t3695: F, t3699: F, t3722: F, t809: F, t819: F, t8339: F) -> (F, F) {
    let t10895 = t812 * t10894;
    let t10897 = t10819 * t253 - 2.0 * t10821 * t819 - 6.0 * t10833 * t809 + 4.0 * t10837 * t809 + 2.0 * t10841 * t809 - t10895 * t809 - t1396 * t8339 + 4.0 * t2401 * t3699 - 2.0 * t2401 * t3722 + 2.0 * t2408 * t3695 - t2426 * t3695;
    (t10895, t10897)
}
