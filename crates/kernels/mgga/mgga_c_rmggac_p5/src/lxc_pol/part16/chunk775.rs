//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 775/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk775<F: Float>(t132: F, t26078: F, t36: F, t4787: F, t638: F, t71: F, t2184: F, t465: F, t7472: F, t7335: F, t7341: F, t20: F, t2018: F, t2021: F, t4720: F) -> (F, F, F, F, F) {
    let t36700 = t638 * t36 * t26078 * t71 * t132 * t4787;
    let t36733 = t465 * t2184;
    let t36734 = t7472 * t36733;
    let t36748 = t7335 * t7341;
    let t36752 = t4720 * t20 * t2018 * t2021;
    (t36700, t36733, t36734, t36748, t36752)
}
