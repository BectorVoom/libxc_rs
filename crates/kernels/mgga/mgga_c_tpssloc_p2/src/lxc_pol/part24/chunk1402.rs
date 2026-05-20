//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1402/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1402<F: Float>(t23366: F, t23384: F, t23582: F, t23333: F, t82431: F, t10167: F, t10170: F, t10182: F, t11010: F, t11085: F, t1956: F, t23317: F, t23323: F, t23327: F, t23329: F, t23346: F, t23402: F, t23581: F, t23722: F, t23725: F, t3026: F, t3169: F, t43619: F, t6687: F, t6700: F, t6771: F, t6776: F, t6816: F, t82441: F, t884: F) -> F {
    let t83316 = t23384 * t23366;
    let t83318 = t23384 * t23582;
    let t83329 = t82431 * t23333;
    let t83341 = -t6771 * t11085 - F::new(3.0) * t3026 * t23722 + F::new(6.0) * t11010 * t6776 - t43619 * t1956 - F::cast_from(0.16449340668482264365e-1_f64) * t83316 + F::cast_from(0.54831135561607547883e-2_f64) * t83318 + F::new(6.0) * t6771 * t10182 - F::new(3.0) * t10170 * t6816 + F::new(12.0) * t3169 * t23725 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t23581 * t23402 - F::cast_from(0.54831135561607547883e-2_f64) * t83329 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23329 * t82441 * t884 - F::new(6.0) * t6771 * t10167 + F::cast_from(0.24125699647107321069e0_f64) * t23323 * t6700 + F::cast_from(0.65797362673929057459e-1_f64) * t23346 * t23317;
    t83341
}
