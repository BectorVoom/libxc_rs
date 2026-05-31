//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2229/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2229<F: Float>(t1888: F, t232: F, t58262: F, t6646: F, t23110: F, t23185: F, t28422: F, t16817: F, t82018: F, t16825: F, t22996: F, t1510: F, t16673: F, t16753: F, t2617: F, t28351: F, t28409: F, t28411: F, t6657: F, t6658: F, t812: F, t87101: F, t87135: F, t92497: F, t98374: F, t98380: F, t98384: F, t98387: F, t98392: F) -> F {
    let t98396 = t1888 * t6646 * t58262 * t232;
    let t98399 = t23185 * t23110 * t28422;
    let t98402 = t1888 * t82018 * t16817;
    let t98405 = t1888 * t22996 * t16825;
    let t98409 = -F::cast_from(2.0_f64) * t812 * t87135 * t1510 - F::cast_from(0.19190897446562641759e-1_f64) * t98374 + t92497 - F::cast_from(2.0_f64) * t2617 * t28351 - t812 * t6657 * t16753 + F::cast_from(0.19190897446562641759e-1_f64) * t98380 - t2617 * t28411 - F::cast_from(0.16449340668482264365e-1_f64) * t98384 - F::cast_from(0.82246703342411321825e-2_f64) * t98387 + F::cast_from(0.9869604401089358619e-1_f64) * t98392 - F::cast_from(0.82246703342411321825e-2_f64) * t98396 + F::cast_from(0.41123351671205660912e-2_f64) * t98399 - F::cast_from(0.49348022005446793095e-1_f64) * t98402 + F::cast_from(0.49348022005446793095e-1_f64) * t98405 + t87101 - t16673 * t6658 - t2617 * t28409;
    t98409
}
