//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1403/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1403<F: Float>(t23323: F, t6683: F, t23357: F, t6680: F, t23494: F, t381: F, t23384: F, t23403: F, t23589: F, t6695: F, t82632: F, t10170: F, t1956: F, t23327: F, t23337: F, t23341: F, t23346: F, t23372: F, t23378: F, t23725: F, t3026: F, t3169: F, t3207: F, t43599: F, t6691: F, t6707: F, t6776: F, t82382: F, t82402: F) -> F {
    let t83342 = t23323 * t6683;
    let t83344 = t6680 * t23357;
    let t83352 = t23494 * t381;
    let t83358 = t23384 * t23403;
    let t83364 = t23384 * t23589;
    let t83368 = t82632 * t6695;
    let t83376 = F::cast_from(0.80418998823691070229e-1_f64) * t83342 + F::cast_from(0.14621636149762012769e-1_f64) * t83344 + F::cast_from(12.0_f64) * t3026 * t23725 - F::cast_from(18.0_f64) * t3169 * t23341 + F::cast_from(6.0_f64) * t3026 * t23378 - F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t83352 * t6691 + F::cast_from(0.43864908449286038307e-1_f64) * t82402 * t23337 - F::cast_from(0.54831135561607547883e-2_f64) * t83358 + F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t23403 - F::cast_from(0.13159472534785811492e0_f64) * t23346 * t23589 + F::cast_from(0.16449340668482264365e-1_f64) * t83364 - F::cast_from(0.24125699647107321069e0_f64) * t82382 * t6707 + F::cast_from(0.54831135561607547884e-2_f64) * t83368 - F::cast_from(3.0_f64) * t23372 * t3207 + F::cast_from(6.0_f64) * t10170 * t6776 - F::cast_from(3.0_f64) * t43599 * t1956;
    t83376
}
