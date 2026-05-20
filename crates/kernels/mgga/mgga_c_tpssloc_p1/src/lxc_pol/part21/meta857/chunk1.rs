//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3109/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3109<F: Float>(t51043: F, t51051: F, t51053: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F) -> F {
    let t64406 = -F::new(0.66228e0) * t51043 - F::cast_from(0.12264444444444444444e0_f64) * t51051 - F::new(0.44152e0) * t51053 + F::new(0.301925e0) * t63355 - F::cast_from(0.40256666666666666666e0_f64) * t63359 + F::cast_from(0.26837777777777777777e0_f64) * t63361 + F::new(0.12077e1) * t63365 - F::new(0.12077e1) * t63370 + F::cast_from(0.33547222222222222222e0_f64) * t63374 + F::cast_from(0.40256666666666666666e1_f64) * t63380 + F::cast_from(0.26837777777777777778e0_f64) * t63382 + F::cast_from(0.80513333333333333333e0_f64) * t63384 - F::new(0.12077e1) * t63388 - F::new(0.72462e1) * t63392 - F::cast_from(0.40256666666666666666e0_f64) * t63396;
    t64406
}
