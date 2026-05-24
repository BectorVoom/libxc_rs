//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1012/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1012<F: Float>(t1971: F, t3351: F, t6400: F, t880: F, t7720: F, t9938: F, t16043: F, t9975: F, t1704: F, t236: F, t35155: F, t498: F) -> (F, F, F, F) {
    let t47156 = t3351 * t1971 * t880 * t6400;
    let t47158 = t7720 * t9938;
    let t47162 = t16043 * t9975;
    let t47167 = t3351 * t35155 * t236 * t1704 * t498;
    (t47156, t47158, t47162, t47167)
}
