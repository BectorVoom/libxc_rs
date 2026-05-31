//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2018/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2018<F: Float>(t22573: F, t6875: F, t111: F, t7002: F, t26555: F, t576: F, t1858: F, t2029: F, t5363: F, t1851: F, t7020: F, t1453: F, t81439: F) -> (F, F, F, F, F, F, F) {
    let t83886 = t6875 * t22573;
    let t83980 = t7002 * t111;
    let t86565 = F::cast_from(2.0_f64) * t576 * t26555;
    let t86567 = F::cast_from(2.0_f64) * t7002 * t1858;
    let t86571 = F::cast_from(2.0_f64) * t5363 * t2029;
    let t86579 = F::cast_from(2.0_f64) * t1851 * t7020;
    let t86586 = t81439 * t1453;
    (t83886, t83980, t86565, t86567, t86571, t86579, t86586)
}
