//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 728/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk728<F: Float>(t7203: F, t899: F, t20: F, t4764: F, t132: F, t1327: F, t20925: F, t253: F, t7321: F, t4765: F, t49: F, t7322: F) -> (F, F, F, F) {
    let t34738 = t899 * t7203;
    let t34747 = t20 * t4764;
    let t34750 = t132 * t1327;
    let t34752 = t253 * t34747 * t7321 * t20925 * t34750;
    let t34753 = F::cast_from(0.10260057759007034251e-5_f64) * t34752;
    let t34755 = t4765 * t7322 * t49;
    (t34738, t34750, t34753, t34755)
}
