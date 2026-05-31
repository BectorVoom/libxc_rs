//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1176/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1176<F: Float>(t19390: F, t607: F, t3966: F, t3990: F, t2274: F, t5398: F, t16558: F, t55: F, t1420: F, t19369: F, t19372: F, t19378: F, t19381: F, t39: F, t3991: F, t3994: F, t51: F, t5408: F, t5411: F, t5416: F, t615: F, t621: F, t9311: F) -> F {
    let t19391 = t19390 * t607;
    let t19394 = t3990 * t3966;
    let t19397 = t2274 * t5398;
    let t19398 = t19397 * t607;
    let t19401 = t55 * t16558;
    let t19404 = -F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t615 * t5408 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t39 * t19369 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t39 * t19372 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t615 * t5411 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t39 * t19378 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t19381 - F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t5416 * t621 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t1420 * t3991 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1420 * t3994 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t51 * t19391 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t51 * t19394 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t19398 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t19401 + t9311;
    t19404
}
