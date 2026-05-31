//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2179/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2179<F: Float>(t80940: F, t80957: F, t80971: F, t91400: F, t91403: F, t91404: F, t93760: F, t97435: F, t97437: F, t97439: F, t97444: F, t97447: F, t97450: F, t97453: F, t97456: F, t97459: F, t97461: F, t97463: F) -> F {
    let t97465 = -F::cast_from(0.48447307312968469024e-2_f64) * t97435 - t97437 / F::cast_from(48.0_f64) + F::cast_from(0.84782787797694820792e-2_f64) * t97439 - t93760 - F::cast_from(0.13565246047631171327e0_f64) * t91400 + t91403 + F::cast_from(0.16956557559538964159e-1_f64) * t91404 - F::cast_from(0.11304371706359309439e-1_f64) * t80940 + F::cast_from(0.14130464632949136799e-2_f64) * t97444 + F::cast_from(0.16956557559538964158e-1_f64) * t97447 + F::cast_from(0.84782787797694820792e-2_f64) * t97450 - t80957 + t80971 - t97453 / F::cast_from(4.0_f64) + t97456 / F::cast_from(8.0_f64) - F::cast_from(0.67826230238155856634e-1_f64) * t97459 - t97461 / F::cast_from(256.0_f64) + F::cast_from(0.14130464632949136799e-2_f64) * t97463;
    t97465
}
