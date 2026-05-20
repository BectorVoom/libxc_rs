//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1110/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1110<F: Float>(t27473: F, t7362: F, t1215: F, t8054: F, t1246: F, t1244: F, t24760: F, t24773: F, t27406: F, t27451: F, t27455: F, t27462: F, t27466: F, t27471: F, t5064: F, t7283: F, t7365: F, t7387: F) -> F {
    let t27474 = t7362 * t27473;
    let t27477 = t8054 * t1215;
    let t27478 = t27477 * t1246;
    let t27480 = -F::cast_from(0.27415567780803773942e-2_f64) * t24760 - F::cast_from(0.91385225936012579807e-3_f64) * t27451 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27455 + F::cast_from(0.73108180748810063843e-2_f64) * t27406 * t7365 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t27462 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t27466 + t5064 * t7387 + t1244 * t27471 - t24773 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t27474 + t1244 * t27478;
    t27480
}
