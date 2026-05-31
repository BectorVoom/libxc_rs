//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2348/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2348<F: Float>(t111: F, t27370: F, t12813: F, t1458: F, t2363: F, t24932: F, t27863: F, t27888: F, t4072: F, t671: F, t7266: F, t85428: F, t90355: F, t90361: F, t90363: F, t90365: F, t90367: F, t90369: F, t94248: F, t96222: F) -> (F, F) {
    let t96238 = t27370 * t111;
    let t96269 = F::cast_from(2.0_f64) * t12813 * t7266 + F::cast_from(2.0_f64) * t1458 * t85428 + F::cast_from(2.0_f64) * t1458 * t94248 + F::cast_from(4.0_f64) * t1458 * t96222 + F::cast_from(2.0_f64) * t2363 * t27863 + F::cast_from(4.0_f64) * t24932 * t4072 + F::cast_from(4.0_f64) * t27888 * t4072 + F::cast_from(4.0_f64) * t671 * t96238 + t90355 + t90361 + t90363 + t90365 + t90367 + t90369;
    (t96238, t96269)
}
