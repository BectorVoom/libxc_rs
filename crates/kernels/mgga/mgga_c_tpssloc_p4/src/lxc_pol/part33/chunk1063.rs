//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1063/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1063<F: Float>(t1241: F, t22393: F, t22113: F, t491: F, t1238: F, t1761: F, t19232: F, t19234: F, t19249: F, t22004: F, t22008: F, t22328: F, t22334: F, t22337: F, t4945: F, t498: F, t5055: F, t6244: F, t6268: F) -> F {
    let t22394 = t1241 * t22393;
    let t22398 = t22113 * t491;
    let t22408 = F::cast_from(6.0_f64) * t1238 * t22004 - F::cast_from(6.0_f64) * t1238 * t22008 - t1238 * t22394 - F::cast_from(3.0_f64) * t1761 * t19232 - F::cast_from(6.0_f64) * t1761 * t19234 - F::cast_from(3.0_f64) * t1761 * t19249 + t22328 * t498 + F::cast_from(3.0_f64) * t22334 * t498 + F::cast_from(3.0_f64) * t22337 * t498 + t22398 * t498 + F::cast_from(6.0_f64) * t4945 * t6244 - F::cast_from(3.0_f64) * t4945 * t6268 + F::cast_from(6.0_f64) * t5055 * t6244 - F::cast_from(3.0_f64) * t5055 * t6268;
    t22408
}
