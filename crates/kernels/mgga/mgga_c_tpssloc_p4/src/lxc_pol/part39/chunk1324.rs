//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1324/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1324<F: Float>(t111: F, t8283: F, t110363: F, t12521: F, t12524: F, t12813: F, t16521: F, t16524: F, t16541: F, t20173: F, t2199: F, t2319: F, t2363: F, t30112: F, t30125: F, t30315: F, t30363: F, t30382: F, t30385: F, t30390: F, t3941: F, t5376: F, t55353: F, t55571: F, t671: F, t8189: F, t8207: F, t8212: F, t8273: F, t8294: F) -> F {
    let t111246 = t8283 * t111;
    let t111284 = F::cast_from(54.0_f64) * t12524 * t30385 + F::cast_from(27.0_f64) * t111246 * t2319 + F::cast_from(0.135e2_f64) * t8207 * t12813 + F::cast_from(27.0_f64) * t30112 * t16541 + F::cast_from(54.0_f64) * t55353 * t8212 + F::cast_from(27.0_f64) * t16521 * t8189 + F::cast_from(54.0_f64) * t16524 * t30125 + F::cast_from(54.0_f64) * t12524 * t30382 + F::cast_from(0.135e2_f64) * t30363 * t2363 + F::cast_from(54.0_f64) * t110363 * t5376 + F::cast_from(54.0_f64) * t12524 * t30390 + F::cast_from(27.0_f64) * t3941 * t2199 * t12813 + F::cast_from(0.135e2_f64) * t12521 * t8273 + F::cast_from(27.0_f64) * t55571 * t8294 + F::cast_from(54.0_f64) * t20173 * t30382 + F::cast_from(54.0_f64) * t20173 * t30385 + F::cast_from(54.0_f64) * t3941 * t30315 * t671 + F::cast_from(27.0_f64) * t3941 * t8273 * t2363;
    t111284
}
