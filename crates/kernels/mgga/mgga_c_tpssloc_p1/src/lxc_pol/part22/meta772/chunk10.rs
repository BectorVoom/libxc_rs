//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2642/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2642<F: Float>(t22337: F, t225: F, t22328: F, t11606: F, t1235: F, t1238: F, t1252: F, t14980: F, t1720: F, t1760: F, t1761: F, t19120: F, t19208: F, t19214: F, t19220: F, t19226: F, t19232: F, t19249: F, t22113: F, t22394: F, t3487: F, t3598: F, t498: F, t5055: F, t5060: F, t5088: F, t5089: F, t6243: F, t6268: F, t65208: F) -> F {
    let t73891 = t22337 * t225;
    let t73900 = t22328 * t225;
    let t73919 = -F::cast_from(18.0_f64) * t11606 * t1238 * t5088 * t6243 + F::cast_from(6.0_f64) * t1238 * t1760 * t19208 * t3598 + t1235 * t22113 * t498 + F::cast_from(3.0_f64) * t1720 * t19120 * t498 - F::cast_from(3.0_f64) * t1252 * t73891 - t1252 * t73900 - F::cast_from(3.0_f64) * t14980 * t6268 - F::cast_from(3.0_f64) * t1761 * t65208 + F::cast_from(12.0_f64) * t19214 * t5055 + F::cast_from(6.0_f64) * t19220 * t5055 - F::cast_from(18.0_f64) * t19226 * t5055 + F::cast_from(6.0_f64) * t19232 * t5060 - F::cast_from(3.0_f64) * t19249 * t5089 - t22394 * t3487;
    t73919
}
