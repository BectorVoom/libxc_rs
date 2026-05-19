//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 625/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk625<F: Float>(t1201: F, t1244: F, t2121: F, t2152: F, t470: F, t7283: F, t7361: F, t7365: F, t7368: F, t7373: F, t7378: F, t7382: F, t7387: F, t7389: F) -> F {
    let t7391 = t7361 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7365 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t7368 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t7378 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t7382 + t1201 * t2152 + t1244 * t7387 + t470 * t7389;
    t7391
}
