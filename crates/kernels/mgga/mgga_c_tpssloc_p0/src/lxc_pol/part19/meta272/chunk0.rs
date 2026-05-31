//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1031/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1031<F: Float>(t25: F, t514: F, t3665: F, t606: F, t3704: F, t1298: F, t2249: F, t9257: F, t28: F, t517: F, t1081: F, t3673: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t11985 = t25 * t25;
    let t11987 = F::cast_from(1.0_f64) / t514 / t11985;
    let t11988 = t3665 * t606;
    let t11991 = t3704 * t606;
    let t11997 = piecewise3::<F>(t26, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11987 * t11988 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11991 * t2249 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1298 * t9257);
    let t11998 = t28 * t28;
    let t12000 = F::cast_from(1.0_f64) / t517 / t11998;
    let t12001 = t3673 * t1081;
    (t11985, t11987, t11988, t11991, t11997, t11998, t12000, t12001)
}
