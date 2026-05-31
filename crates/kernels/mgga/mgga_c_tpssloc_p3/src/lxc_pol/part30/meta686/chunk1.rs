//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2168/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2168<F: Float>(t1361: F, t22690: F, t6330: F, t80840: F, t22792: F, t6347: F, t80900: F, t80915: F, t91387: F, t93757: F, t97394: F, t97398: F, t97400: F, t97402: F, t97404: F, t97407: F, t97410: F, t97412: F, t97414: F, t97416: F, t97419: F, t97423: F) -> F {
    let t97427 = t80840 * t22690 * t1361 * t6330;
    let t97431 = t22792 * t22690 * t1361 * t6347;
    let t97433 = -t80900 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t97394 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t80915 - F::cast_from(0.20186378047070195427e-3_f64) * t97398 - F::cast_from(0.28260929265898273598e-2_f64) * t97400 - t91387 - t93757 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t97402 - F::cast_from(0.59347951458386374554e-1_f64) * t97404 - F::cast_from(0.16956557559538964158e-1_f64) * t97407 + F::cast_from(0.24223653656484234512e-2_f64) * t97410 - t97412 / F::cast_from(192.0_f64) + t97414 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t97416 + t97419 / F::cast_from(16.0_f64) - F::cast_from(0.12111826828242117256e-2_f64) * t97423 - F::cast_from(0.14130464632949136799e-2_f64) * t97427 + F::cast_from(0.20186378047070195427e-3_f64) * t97431;
    t97433
}
