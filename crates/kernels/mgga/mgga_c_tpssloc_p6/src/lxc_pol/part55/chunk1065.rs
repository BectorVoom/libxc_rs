//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1065/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1065<F: Float>(t12524: F, t8319: F, t20173: F, t1873: F, t6534: F, t3941: F, t3938: F, t8326: F, t671: F, t649: F, t89: F, t88: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31277 = F::cast_from(27.0_f64) * t12524 * t8319;
    let t31279 = F::cast_from(27.0_f64) * t20173 * t8319;
    let t31280 = t1873 * t6534;
    let t31282 = F::cast_from(54.0_f64) * t3941 * t31280;
    let t31283 = t3938 * t8326;
    let t31284 = F::cast_from(0.135e2_f64) * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = F::cast_from(27.0_f64) * t31286;
    let t31537 = t649 * t1873;
    let t31540 = t89 * t6534;
    let t31717 = t88 * t6534;
    (t31277, t31279, t31280, t31282, t31284, t31285, t31287, t31537, t31540, t31717)
}
