//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1208/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1208<F: Float>(t119274: F, t119303: F, t119324: F, t119349: F, t23384: F, t32928: F, t1003: F, t1022: F, t1058: F, t1060: F, t113528: F, t119238: F, t23327: F, t23346: F, t25470: F, t30878: F, t30895: F, t3180: F, t32931: F, t32938: F, t32944: F, t32961: F, t32962: F, t353: F, t383: F, t4669: F, t6687: F, t986: F) -> (F, F) {
    let t119351 = t119274 + t119303 + t119324 + t119349;
    let t119357 = t23384 * t32928;
    let t119366 = F::cast_from(0.18277045187202515961e-2_f64) * t113528 + t3180 * t32944 + F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t32931 - F::cast_from(0.54831135561607547883e-2_f64) * t119238 + t1003 * t32962 + t353 * t383 * t119351 - F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t25470 * t30878 + F::cast_from(0.18277045187202515961e-2_f64) * t119357 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t32938 + t4669 * t30895 + t1058 * t32961 * t1022 * t1060;
    (t119351, t119366)
}
