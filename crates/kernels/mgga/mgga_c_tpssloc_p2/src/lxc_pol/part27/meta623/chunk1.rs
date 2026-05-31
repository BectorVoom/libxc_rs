//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2103/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2103<F: Float>(t26555: F, t576: F, t1858: F, t7002: F, t2029: F, t5363: F, t1851: F, t7020: F, t16507: F, t16546: F, t1852: F, t2023: F, t23863: F, t23901: F, t3946: F, t5381: F, t7003: F, t7759: F, t80593: F, t80597: F, t84024: F) -> F {
    let t86565 = F::cast_from(2.0_f64) * t576 * t26555;
    let t86567 = F::cast_from(2.0_f64) * t7002 * t1858;
    let t86571 = F::cast_from(2.0_f64) * t5363 * t2029;
    let t86579 = F::cast_from(2.0_f64) * t1851 * t7020;
    let t86580 = t16507 * t2029 + t16546 * t2023 + t1852 * t23901 + t1858 * t23863 + t3946 * t7759 + F::cast_from(2.0_f64) * t5381 * t7003 + t80593 + t80597 + F::cast_from(2.0_f64) * t84024 + t86565 + t86567 + t86571 + t86579;
    t86580
}
