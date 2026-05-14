//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1274/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1274<F: Float>(t1206: F, t5458: F, t18547: F, t19580: F, t6242: F, t7309: F, t19582: F, t21106: F, t508: F, t1760: F, t5709: F, t14001: F, t196: F, t197: F, t1779: F, t21253: F, t5755: F) -> (F, F, F, F, F) {
    let t68958 = t5458 * t1206;
    let t68961 = 6.0 * t18547 * t19580 * t68958;
    let t68967 = t6242 * t7309;
    let t68969 = 4.0 * t68967 * t19582;
    let t68970 = t508 * t21106;
    let t68973 = 3.0 * t1760 * t68970 * t5709;
    let t68975 = t14001 * t196 * t197;
    let t68976 = t68975 * t1779;
    let t68977 = t21253 * t5755;
    (t68961, t68969, t68973, t68976, t68977)
}
