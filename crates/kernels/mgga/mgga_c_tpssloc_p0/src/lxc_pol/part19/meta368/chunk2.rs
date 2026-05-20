//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1358/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1358<F: Float>(t1032: F, t10375: F, t370: F, t374: F, t376: F, t9697: F, t10908: F, t3109: F, t1036: F, t10446: F, t1004: F, t10249: F, t10413: F, t10445: F, t14220: F, t2979: F, t3070: F, t3071: F, t35: F, t354: F, t364: F, t378: F, t41649: F, t43226: F, t43228: F, t43233: F, t43235: F, t43241: F, t43246: F, t6720: F, t973: F) -> F {
    let t43248 = t1032 * t10375;
    let t43253 = F::new(7.0) / F::new(31104.0) * t370 * t374 * t9697 * t376;
    let t43254 = t3109 * t10908;
    let t43262 = t10446 * t1036;
    let t43267 = t43226 / F::new(576.0) + t43228 / F::new(216.0) - t973 * t2979 * t41649 / F::new(6.0) - t43233 / F::new(384.0) - t10413 * t3071 * t43235 * t14220 / F::new(384.0) - t3070 * t3071 * t10249 * t43241 / F::new(192.0) - t43246 / F::new(72.0) - t43248 / F::new(486.0) - t43253 - t43254 / F::new(72.0) + F::new(5225.0) / F::new(7776.0) * t354 * t364 / t35 / t6720 * t378 - F::new(209.0) / F::new(972.0) * t43262 - F::new(209.0) / F::new(648.0) * t1004 * t10445 * t378;
    t43267
}
