//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1227/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1227<F: Float>(t114: F, t1689: F, t21180: F, t3493: F, t6106: F, t18397: F, t4645: F, t4669: F, t5527: F, t18393: F, t20315: F) -> (F, F, F) {
    let t115 = 1.0 < t114;
    let t21182 = 4.0 * t21180 * t1689;
    let t21184 = 4.0 * t3493 * t6106;
    let t21185 = t18397 * t4645;
    let t21187 = t5527 * t4669;
    let t21190 = piecewise3(t115, 0.0, t18393 + t20315 + t21185 / 4.0 - t21187 / 8.0);
    (t21182, t21184, t21190)
}
