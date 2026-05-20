//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3100/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3100<F: Float>(t51043: F, t51051: F, t51053: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F) -> F {
    let t64229 = -F::cast_from(0.83356000000000000002e0_f64) * t51043 - F::cast_from(0.15436296296296296297e0_f64) * t51051 - F::cast_from(0.55570666666666666668e0_f64) * t51053 + F::new(0.516475e0) * t63355 - F::cast_from(0.68863333333333333334e0_f64) * t63359 + F::cast_from(0.45908888888888888889e0_f64) * t63361 + F::new(0.20659e1) * t63365 - F::new(0.20659e1) * t63370 + F::cast_from(0.57386111111111111112e0_f64) * t63374 + F::cast_from(0.68863333333333333334e1_f64) * t63380 + F::cast_from(0.45908888888888888889e0_f64) * t63382 + F::cast_from(0.13772666666666666666e1_f64) * t63384 - F::new(0.20659e1) * t63388 - F::new(0.123954e2) * t63392 - F::cast_from(0.68863333333333333334e0_f64) * t63396;
    t64229
}
