//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 696/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk696<F: Float>(t15405: F, t7255: F, t3351: F, t498: F, t515: F, t7248: F, t8975: F, t1970: F, t1971: F, t209: F, t2123: F, t605: F, t14124: F, t14125: F, t236: F, t29122: F) -> (F, F, F, F) {
    let t74421 = t7255 * t15405;
    let t74426 = t3351 * t7248 * t515 * t8975 * t498;
    let t74432 = t1970 * t1971 * t515 * t2123 * t605 * t209;
    let t74436 = t14124 * t14125 * t236 * t29122;
    (t74421, t74426, t74432, t74436)
}
