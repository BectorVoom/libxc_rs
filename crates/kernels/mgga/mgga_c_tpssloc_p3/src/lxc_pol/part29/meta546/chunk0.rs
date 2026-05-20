//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1944/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1944<F: Float>(t24615: F, t5059: F, t7300: F, t5088: F, t7301: F, t2144: F, t4940: F, t1238: F, t24575: F, t24577: F, t24587: F, t27383: F, t27389: F, t27392: F, t27396: F, t27401: F, t27403: F, t27406: F, t3593: F, t498: F, t7283: F, t7291: F, t7303: F, t8061: F) -> (F, F, F, F, F, F) {
    let t27411 = t24615 * t5059;
    let t27412 = t7300 * t27411;
    let t27415 = t7301 * t5088;
    let t27416 = t7300 * t27415;
    let t27419 = t4940 * t2144;
    let t27421 = F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27383 - F::cast_from(0.27415567780803773942e-2_f64) * t24575 - F::cast_from(0.27415567780803773942e-2_f64) * t24577 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t27389 + F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27392 - t24587 + F::new(2.0) * t1238 * t27396 + F::new(2.0) * t3593 * t8061 - F::cast_from(0.91385225936012579807e-3_f64) * t27401 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27403 + F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t7303 + F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t7291 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t27412 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27416 + t27419 * t498;
    (t27411, t27412, t27415, t27416, t27419, t27421)
}
