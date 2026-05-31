//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2182/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2182<F: Float>(t1458: F, t2311: F, t1873: F, t22479: F, t7676: F, t7467: F, t9348: F, t45632: F, t12734: F, t2314: F, t26135: F, t12739: F) -> (F, F, F, F, F, F, F, F) {
    let t90381 = t2311 * t1458;
    let t90383 = F::cast_from(2.0_f64) * t90381 * t1873;
    let t90385 = F::cast_from(2.0_f64) * t7676 * t22479;
    let t90387 = F::cast_from(2.0_f64) * t9348 * t7467;
    let t90399 = F::cast_from(2.0_f64) * t45632 * t1873;
    let t90404 = F::cast_from(4.0_f64) * t12734 * t7467;
    let t90406 = F::cast_from(4.0_f64) * t2314 * t26135;
    let t90408 = F::cast_from(2.0_f64) * t12739 * t7467;
    (t90381, t90383, t90385, t90387, t90399, t90404, t90406, t90408)
}
