//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2008/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2008<F: Float>(t1375: F, t16436: F, t2091: F, t3887: F, t80689: F, t90521: F, t90527: F, t90530: F, t90539: F, t93350: F, t93353: F, t93359: F, t93361: F, t93362: F) -> F {
    let t93363 = -F::cast_from(0.25587863262083522346e0_f64) * t90521 - t93350 - F::cast_from(0.3289868133696452873e-1_f64) * t90527 - F::cast_from(0.6579736267392905746e-1_f64) * t90530 + t93353 + F::new(2.0) * t1375 * t3887 * t2091 * t16436 + F::cast_from(0.3289868133696452873e-1_f64) * t90539 + t93359 + F::cast_from(0.38381794893125283518e-1_f64) * t80689 + t93361 - t93362;
    t93363
}
