//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1455/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1455<F: Float>(t103218: F, t103226: F, t103332: F, t104556: F, t109697: F, t1409: F, t1761: F, t2123: F, t2144: F, t2155: F, t22008: F, t22034: F, t22113: F, t24589: F, t24601: F, t27406: F, t27792: F, t29532: F, t29551: F, t29694: F, t29809: F, t466: F, t4945: F, t498: F, t6243: F, t6268: F, t7283: F, t7351: F, t73613: F, t73900: F, t8002: F, t8003: F, t94332: F, t94395: F) -> F {
    let t109778 = F::new(12.0) * t4945 * t29532 - F::cast_from(0.80418998823691070229e-1_f64) * t103218 * t8003 - F::cast_from(0.54831135561607547883e-2_f64) * t103332 - F::new(3.0) * t27792 * t6268 - t73900 * t2155 + t22113 * t2144 * t498 - t73613 * t2155 + t466 * t109697 * t498 + F::cast_from(0.82246703342411321826e-2_f64) * t24589 * t103226 * t8002 - F::cast_from(0.43864908449286038307e-1_f64) * t94395 * t29809 - F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t24601 * t94332 * t1409 * t6243 + F::cast_from(0.65797362673929057459e-1_f64) * t27406 * t29551 - F::new(6.0) * t104556 * t1761 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t22034 * t2123 - F::new(6.0) * t7351 * t22008 + F::cast_from(0.43864908449286038307e-1_f64) * t27406 * t29694;
    t109778
}
