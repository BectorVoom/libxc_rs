//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 556/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk556<F: Float>(t2021: F, t7491: F, t511: F, t892: F, t504: F, t880: F, t2144: F, t2131: F, t942: F, t1320: F, t1322: F, t1325: F) -> (F, F, F, F, F, F, F) {
    let t7492 = t7491 * t2021;
    let t7494 = t892 * t511;
    let t7501 = t504 * t880;
    let t7508 = t504 * t2144;
    let t7536 = t942 * t2131;
    let t7537 = F::cast_from(0.4726e1_f64) * t7536;
    let t7551 = t1320 * t1322;
    let t7552 = t7551 * t1325;
    (t7492, t7494, t7501, t7508, t7537, t7551, t7552)
}
