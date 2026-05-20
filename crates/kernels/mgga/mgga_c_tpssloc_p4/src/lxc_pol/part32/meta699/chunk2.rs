//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2187/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2187<F: Float>(t1375: F, t1386: F, t16460: F, t19647: F, t20044: F, t20050: F, t20060: F, t22670: F, t26224: F, t26225: F, t26371: F, t26472: F, t26482: F, t3887: F, t5215: F, t5321: F, t5353: F, t6461: F, t6963: F, t6993: F, t7749: F, t7750: F, t81311: F, t90696: F, t90724: F, t97558: F, t97571: F, t97573: F, t97577: F, t97583: F, t97588: F, t97599: F, t97604: F) -> F {
    let t97607 = F::new(4.0) * t5215 * t26371 - t97558 * t1386 - F::new(12.0) * t26224 * t26225 * t19647 - F::new(2.0) * t5321 * t26472 - t20060 * t6993 + F::new(2.0) * t20044 * t6963 - F::new(2.0) * t16460 * t7750 + t90724 - F::cast_from(0.82246703342411321825e-2_f64) * t97571 + F::cast_from(0.38381794893125283518e-1_f64) * t97573 + F::cast_from(0.3289868133696452873e-1_f64) * t97577 + F::new(4.0) * t5215 * t26482 - F::cast_from(0.6579736267392905746e-1_f64) * t97583 - F::cast_from(0.19739208802178717238e0_f64) * t97588 - t22670 * t6461 + F::new(4.0) * t1375 * t3887 * t7749 * t5353 + F::new(24.0) * t26224 * t90696 * t20050 - F::cast_from(0.41123351671205660912e-2_f64) * t97599 + F::cast_from(0.82246703342411321825e-2_f64) * t97604 - F::cast_from(0.82246703342411321824e-2_f64) * t81311;
    t97607
}
