//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 966/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk966<F: Float>(t29286: F, t539: F, t1807: F, t7918: F, t2085: F, t6361: F, t12021: F, t2091: F, t6439: F, t1842: F, t7936: F, t3887: F, t1375: F, t1843: F, t24071: F, t26184: F, t26198: F, t26200: F, t26345: F, t27009: F, t27068: F, t28118: F, t28193: F, t28196: F, t28201: F, t5321: F, t568: F, t7925: F) -> (F, F, F, F, F, F) {
    let t29287 = t539 * t29286;
    let t29290 = t1807 * t7918;
    let t29293 = t6361 * t2085;
    let t29299 = t12021 * t2091 * t6439;
    let t29310 = t7936 * t1842;
    let t29311 = t3887 * t29310;
    let t29314 = 0.15352717957250113407e0 * t26184 + 0.3289868133696452873e-1 * t26198 + t29287 * t568 + 0.76763589786250567036e-1 * t26200 + 2.0 * t29290 * t568 + t29293 * t568 - t24071 + 0.6579736267392905746e-1 * t28118 - 2.0 * t27068 * t1843 - 6.0 * t1375 * t29299 + 0.16449340668482264365e-1 * t26345 + 0.9869604401089358619e-1 * t28193 - 0.3289868133696452873e-1 * t28196 - 2.0 * t27009 * t1843 + 0.16449340668482264365e-1 * t28201 + 4.0 * t5321 * t7925 + 4.0 * t1375 * t29311;
    (t29287, t29290, t29293, t29299, t29311, t29314)
}
