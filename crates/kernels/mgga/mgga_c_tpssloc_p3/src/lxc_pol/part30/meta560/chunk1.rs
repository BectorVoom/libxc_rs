//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1921/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1921<F: Float>(t1528: F, t17052: F, t17092: F, t1912: F, t25036: F, t25188: F, t25348: F, t259: F, t26591: F, t28265: F, t28269: F, t28274: F, t28278: F, t28282: F, t28289: F, t28296: F, t28300: F, t4147: F, t4268: F, t7517: F, t7538: F) -> F {
    let t28304 = -F::cast_from(0.82246703342411321824e-2_f64) * t25036 + F::cast_from(4.0_f64) * t4268 * t7517 - F::cast_from(0.82246703342411321825e-2_f64) * t28265 + F::cast_from(0.3289868133696452873e-1_f64) * t28269 - t26591 + F::cast_from(0.82246703342411321825e-2_f64) * t28274 - F::cast_from(0.16449340668482264365e-1_f64) * t28278 - F::cast_from(2.0_f64) * t25348 * t1528 + t28282 * t259 + F::cast_from(4.0_f64) * t4147 * t7517 - F::cast_from(2.0_f64) * t17092 * t1912 - F::cast_from(0.3289868133696452873e-1_f64) * t28289 - F::cast_from(2.0_f64) * t4147 * t7538 - t17052 * t1912 + F::cast_from(0.16449340668482264365e-1_f64) * t28296 + F::cast_from(0.49348022005446793095e-1_f64) * t28300 - F::cast_from(2.0_f64) * t25188 * t1528;
    t28304
}
