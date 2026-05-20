//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2011/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2011<F: Float>(t12020: F, t7936: F, t16022: F, t20029: F, t26224: F, t5325: F, t7214: F, t7937: F, t90493: F, t90496: F, t90498: F, t90503: F, t93306: F, t93309: F, t93310: F, t93311: F, t93333: F, t96848: F, t96851: F, t96854: F, t96857: F, t96866: F, t96868: F, t96873: F, t96878: F) -> F {
    let t102466 = t12020 * t7936;
    let t102475 = -F::cast_from(0.49348022005446793095e-1_f64) * t96848 + F::cast_from(0.3289868133696452873e-1_f64) * t96851 - F::new(2.0) * t16022 * t7937 + t93306 + F::cast_from(0.19739208802178717238e0_f64) * t96854 + t93309 + t93310 - t93311 - F::cast_from(0.16449340668482264365e-1_f64) * t96857 - F::cast_from(0.3289868133696452873e-1_f64) * t96866 + F::cast_from(0.38381794893125283518e-1_f64) * t96868 + F::cast_from(0.3289868133696452873e-1_f64) * t96873 - F::new(12.0) * t26224 * t102466 * t5325 - F::new(2.0) * t20029 * t7214 + F::cast_from(0.82246703342411321825e-2_f64) * t96878 - t90493 - t90496 - F::cast_from(0.46058153871750340221e0_f64) * t90498 - t93333 + F::cast_from(0.25587863262083522345e0_f64) * t90503;
    t102475
}
