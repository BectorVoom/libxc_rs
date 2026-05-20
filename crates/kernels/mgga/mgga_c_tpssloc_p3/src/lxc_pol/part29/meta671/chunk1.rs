//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2245/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2245<F: Float>(t91344: F, t26245: F, t80783: F, t80870: F, t80872: F, t91304: F, t91305: F, t91311: F, t91312: F, t91314: F, t91317: F, t91319: F, t91321: F, t91323: F, t91328: F, t91330: F, t91333: F, t91336: F, t91340: F) -> F {
    let t91345 = F::cast_from(0.28260929265898273598e-2_f64) * t91344;
    let t91346 = t80783 * t26245;
    let t91348 = -t91304 + F::new(119.0) / F::new(6912.0) * t91305 + t91311 - F::cast_from(0.52708876011794399171e-3_f64) * t91312 - t91314 + F::new(7.0) / F::new(288.0) * t80870 + F::new(7.0) / F::new(576.0) * t80872 + F::new(5.0) / F::new(192.0) * t91317 + F::new(5.0) / F::new(192.0) * t91319 + F::new(5.0) / F::new(384.0) * t91321 + F::cast_from(0.10093189023535097714e-3_f64) * t91323 + t91328 + F::cast_from(0.16956557559538964158e-1_f64) * t91330 + F::cast_from(0.84782787797694820792e-2_f64) * t91333 - F::cast_from(0.20186378047070195427e-3_f64) * t91336 + F::cast_from(0.12111826828242117256e-2_f64) * t91340 - t91345 + F::cast_from(0.16821981705891829522e-4_f64) * t91346;
    t91348
}
