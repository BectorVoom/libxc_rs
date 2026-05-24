//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 560/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk560<F: Float>(t236: F, t7467: F, t3352: F, t1970: F, t1976: F, t5542: F) -> (F, F, F) {
    let t7468 = t236 * t7467;
    let t7469 = t3352 * t7468;
    let t7470 = t1970 * t7469;
    let t7471 = F::cast_from(0.25538759935978703638e-4_f64) * t7470;
    let t7472 = t1976 * t5542;
    (t7469, t7471, t7472)
}
