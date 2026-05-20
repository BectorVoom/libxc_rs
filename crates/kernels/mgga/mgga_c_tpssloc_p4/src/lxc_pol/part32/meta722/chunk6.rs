//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2305/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2305<F: Float>(t2147: F, t8034: F, t29624: F, t7327: F, t103422: F, t1653: F, t18241: F, t19128: F, t24858: F, t27406: F, t27454: F, t27462: F, t27549: F, t27552: F, t29720: F, t3604: F, t5979: F, t7283: F, t7362: F, t7363: F, t7373: F, t7375: F, t7376: F, t7377: F, t94911: F, t94941: F, t94947: F, t95794: F) -> (F, F) {
    let t103683 = t8034 * t2147;
    let t103687 = t29624 * t7327;
    let t103693 = -t94911 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7362 * t24858 * t5979 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t27462 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7362 * t7363 * t18241 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t7362 * t95794 * t1653 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t7375 * t19128 * t7376 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t103422 * t27454 - F::cast_from(0.73108180748810063845e-2_f64) * t27549 * t103683 * t27552 - F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t103687 * t7377 - t94941 + F::new(2.0) * t3604 * t29720 - t94947;
    (t103683, t103693)
}
