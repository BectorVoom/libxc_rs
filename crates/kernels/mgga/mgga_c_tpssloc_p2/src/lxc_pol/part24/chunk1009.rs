//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1009/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1009<F: Float>(t11177: F, t11365: F, t11366: F, t1138: F, t11400: F, t11405: F, t11409: F, t11410: F, t11415: F, t11420: F, t11421: F, t11426: F, t11429: F, t11430: F, t11434: F, t11437: F, t11441: F, t11455: F, t11472: F, t1148: F, t3327: F, t3332: F, t3352: F, t3357: F, t3360: F, t3376: F, t3401: F, t436: F) -> F {
    let t11473 = -F::cast_from(0.10389515463408878255e3_f64) * t11365 * t11366 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t11400 + t11405 - t11409 + F::new(3.0) * t11410 * t1138 + F::new(3.0) * t3327 * t3352 + F::cast_from(0.96491876992155210402e2_f64) * t11415 * t3360 - F::cast_from(0.19298375398431042081e3_f64) * t11420 * t11421 + t11426 - t11429 - F::cast_from(0.35089341735807877242e1_f64) * t3376 * t11430 + F::cast_from(0.51947577317044391277e2_f64) * t3401 * t11434 - F::new(6.0) * t3332 * t11437 + F::cast_from(0.96491876992155210402e2_f64) * t3357 * t11441 - F::new(0.310907e-1) * t11455 * t436 - F::cast_from(0.19751673498613801407e-1_f64) * t11177 + t11472;
    t11473
}
