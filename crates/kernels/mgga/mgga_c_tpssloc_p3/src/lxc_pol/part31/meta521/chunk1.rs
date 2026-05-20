//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1732/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1732<F: Float>(t29310: F, t3887: F, t1375: F, t1843: F, t24071: F, t26184: F, t26198: F, t26200: F, t26345: F, t27009: F, t27068: F, t28118: F, t28193: F, t28196: F, t28201: F, t29287: F, t29290: F, t29293: F, t29299: F, t5321: F, t568: F, t7925: F) -> (F, F) {
    let t29311 = t3887 * t29310;
    let t29314 = F::cast_from(0.15352717957250113407e0_f64) * t26184 + F::cast_from(0.3289868133696452873e-1_f64) * t26198 + t29287 * t568 + F::cast_from(0.76763589786250567036e-1_f64) * t26200 + F::new(2.0) * t29290 * t568 + t29293 * t568 - t24071 + F::cast_from(0.6579736267392905746e-1_f64) * t28118 - F::new(2.0) * t27068 * t1843 - F::new(6.0) * t1375 * t29299 + F::cast_from(0.16449340668482264365e-1_f64) * t26345 + F::cast_from(0.9869604401089358619e-1_f64) * t28193 - F::cast_from(0.3289868133696452873e-1_f64) * t28196 - F::new(2.0) * t27009 * t1843 + F::cast_from(0.16449340668482264365e-1_f64) * t28201 + F::new(4.0) * t5321 * t7925 + F::new(4.0) * t1375 * t29311;
    (t29311, t29314)
}
