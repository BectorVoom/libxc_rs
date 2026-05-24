//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 568/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk568<F: Float>(t2141: F, t7501: F, t649: F, t848: F, t27: F, t2139: F, t2144: F, t504: F) -> (F, F, F, F) {
    let t7502 = t7501 * t2141;
    let t7503 = F::cast_from(0.27274661654245341728e-1_f64) * t7502;
    let t7504 = t649 * t848;
    let t7505 = t27 * t7504;
    let t7506 = t2139 * t7505;
    let t7507 = F::cast_from(0.13637330827122670864e-1_f64) * t7506;
    let t7508 = t504 * t2144;
    (t7503, t7505, t7507, t7508)
}
