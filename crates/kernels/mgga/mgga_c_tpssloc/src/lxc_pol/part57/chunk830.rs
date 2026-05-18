//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 830/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk830<F: Float>(t29310: F, t3887: F, t1375: F, t1843: F, t24071: F, t26184: F, t26198: F, t26200: F, t26345: F, t27009: F, t27068: F, t28118: F, t28193: F, t28196: F, t28201: F, t29287: F, t29290: F, t29293: F, t29299: F, t5321: F, t568: F, t7925: F) -> (F, F) {
    let t29311 = t3887 * t29310;
    let t29314 = F::new(0.15352717957250113407e0) * t26184 + F::new(0.3289868133696452873e-1) * t26198 + t29287 * t568 + F::new(0.76763589786250567036e-1) * t26200 + F::new(2.0) * t29290 * t568 + t29293 * t568 - t24071 + F::new(0.6579736267392905746e-1) * t28118 - F::new(2.0) * t27068 * t1843 - F::new(6.0) * t1375 * t29299 + F::new(0.16449340668482264365e-1) * t26345 + F::new(0.9869604401089358619e-1) * t28193 - F::new(0.3289868133696452873e-1) * t28196 - F::new(2.0) * t27009 * t1843 + F::new(0.16449340668482264365e-1) * t28201 + F::new(4.0) * t5321 * t7925 + F::new(4.0) * t1375 * t29311;
    (t29311, t29314)
}
