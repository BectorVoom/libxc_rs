//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2297/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2297<F: Float>(t3597: F, t6243: F, t6238: F, t7299: F, t1090: F, t18241: F, t19120: F, t19214: F, t19226: F, t2121: F, t2155: F, t225: F, t24589: F, t24601: F, t24880: F, t27403: F, t27406: F, t27438: F, t29678: F, t29798: F, t3487: F, t462: F, t497: F, t6244: F, t66845: F, t7283: F, t7285: F, t7286: F, t7296: F, t7302: F, t7351: F, t94395: F, t94628: F, t94631: F) -> (F, F) {
    let t103345 = t3597 * t6243;
    let t103363 = t7299 * t6238;
    let t103377 = -F::cast_from(6.0_f64) * t7351 * t19226 - F::cast_from(0.54831135561607547883e-2_f64) * t24589 * t24601 * t103345 * t1090 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t27438 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7285 * t7286 * t18241 - F::cast_from(6.0_f64) * t3487 * t29798 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27403 + t94628 + F::cast_from(4.0_f64) * t7351 * t19214 + F::cast_from(0.48738787165873375897e-2_f64) * t94631 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t103363 * t7302 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t462 * t19120 * t225 * t497 - t66845 * t2155 + F::cast_from(2.0_f64) * t24880 * t6244 + F::cast_from(0.80418998823691070228e-1_f64) * t29678 * t7296;
    (t103345, t103377)
}
