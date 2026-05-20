//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2332/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2332<F: Float>(t210: F, t29584: F, t27683: F, t27710: F, t1198: F, t27684: F, t27692: F, t27711: F, t6192: F, t7331: F, t8040: F, t86330: F, t86348: F, t86350: F, t95323: F, t95556: F, t95587: F, t95590: F, t95593: F, t95617: F) -> F {
    let t104410 = t29584 * t210;
    let t104413 = t27710 * t27683;
    let t104424 = t86348 / F::new(10368.0) - t86350 / F::new(6912.0) + t95587 - t95590 - t95593 - t95617 - t86330 * t6192 / F::new(1152.0) - F::new(11.0) / F::new(324.0) * t104410 * t1198 + F::cast_from(0.16149102437656156342e-2_f64) * t104413 * t7331 - F::cast_from(0.16149102437656156342e-2_f64) * t27711 * t27692 + F::cast_from(0.16149102437656156342e-2_f64) * t95323 * t8040 - F::cast_from(0.20186378047070195428e-3_f64) * t95556 * t8040 - F::cast_from(0.20186378047070195428e-3_f64) * t27684 * t27692;
    t104424
}
