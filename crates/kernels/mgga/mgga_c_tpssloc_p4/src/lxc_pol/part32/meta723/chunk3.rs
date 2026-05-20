//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2309/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2309<F: Float>(t24826: F, t29716: F, t103218: F, t103615: F, t103707: F, t1216: F, t24745: F, t27406: F, t27453: F, t27460: F, t27481: F, t27484: F, t27498: F, t3610: F, t3612: F, t7283: F, t7368: F, t85918: F, t85941: F, t85952: F, t85963: F, t94858: F, t94874: F, t95069: F) -> F {
    let t103810 = t24826 * t29716;
    let t103829 = -F::cast_from(0.18277045187202515961e-2_f64) * t85918 + F::cast_from(0.82246703342411321825e-2_f64) * t85963 * t94874 * t103615 * t1216 - F::cast_from(0.54831135561607547883e-2_f64) * t103810 - t95069 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t27453 * t24745 * t27460 - F::cast_from(0.18277045187202515961e-2_f64) * t85941 - F::cast_from(0.80418998823691070228e-1_f64) * t103218 * t7368 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27481 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27484 + F::cast_from(0.6092348395734171987e-3_f64) * t85952 + F::new(2.0) * t3610 * t103707 * t3612 + F::cast_from(0.43864908449286038306e-1_f64) * t94858 * t27498;
    t103829
}
