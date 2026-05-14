//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1284/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1284<F: Float>(t6242: F, t7309: F, t19582: F, t21106: F, t508: F, t1760: F, t5709: F, t14001: F, t196: F, t197: F, t1779: F, t21253: F, t5755: F, t21108: F, t5706: F, t19632: F, t6243: F) -> (F, F, F, F, F, F) {
    let t68967 = t6242 * t7309;
    let t68969 = 4.0 * t68967 * t19582;
    let t68970 = t508 * t21106;
    let t68973 = 3.0 * t1760 * t68970 * t5709;
    let t68975 = t14001 * t196 * t197;
    let t68976 = t68975 * t1779;
    let t68977 = t21253 * t5755;
    let t68980 = t5706 * t21108;
    let t68988 = 6.0 * t6243 * t19632;
    (t68969, t68973, t68976, t68977, t68980, t68988)
}
