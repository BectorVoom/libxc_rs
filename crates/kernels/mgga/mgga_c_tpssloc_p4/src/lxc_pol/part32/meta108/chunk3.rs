//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 667/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk667<F: Float>(t2369: F, t2512: F, t118: F, t168: F, t181: F, t2393: F, t2408: F, t2417: F, t2423: F, t2426: F, t2454: F, t2460: F, t2462: F, t2472: F, t2477: F, t2480: F, t2486: F, t2490: F, t2494: F, t2495: F, t2505: F, t2510: F, t268: F, t725: F, t732: F, t740: F, t747: F) -> (F, F) {
    let t2513 = t2369 * t2512;
    let t2516 = -F::cast_from(0.70983522622222222221e-3_f64) * t118 * t2393 * t168 - F::cast_from(0.34246666666666666666e-1_f64) * t268 * t2454 * t732 - F::cast_from(2.0_f64) * t2460 * t2462 + F::cast_from(1.0_f64) * t725 * t2472 + F::cast_from(0.32163958997385070134e2_f64) * t2477 * t2480 + t2426 + t2486 + t2423 - t2408 - t2417 - F::cast_from(0.24415263074675393405e-3_f64) * t118 * t2393 * t181 - F::cast_from(0.10843581300301739842e-1_f64) * t268 * t2490 * t747 - F::cast_from(0.11696447245269292414e1_f64) * t2494 * t2495 + F::cast_from(0.5848223622634646207e0_f64) * t740 * t2505 + F::cast_from(0.17315859105681463759e2_f64) * t2510 * t2513;
    (t2513, t2516)
}
