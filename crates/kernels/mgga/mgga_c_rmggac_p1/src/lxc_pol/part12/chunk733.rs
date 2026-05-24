//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 733/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk733<F: Float>(t7279: F, t7501: F, t2084: F, t2139: F, t27: F, t848: F, t2189: F, t7228: F, t3350: F) -> (F, F, F, F) {
    let t34822 = t7501 * t7279;
    let t34826 = t2139 * t27 * t2084 * t848;
    let t34846 = t2189 * t7228;
    let t34847 = t34846 * t3350;
    (t34822, t34826, t34846, t34847)
}
