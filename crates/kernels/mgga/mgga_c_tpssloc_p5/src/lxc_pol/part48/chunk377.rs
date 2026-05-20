//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 377/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk377<F: Float>(t2504: F, t746: F, t738: F, t172: F, t180: F, t2369: F, t118: F, t168: F, t181: F, t2393: F, t2408: F, t2417: F, t2423: F, t2426: F, t2454: F, t2460: F, t2462: F, t2472: F, t2477: F, t2480: F, t2486: F, t2490: F, t2494: F, t2495: F, t268: F, t725: F, t732: F, t740: F, t747: F) -> (F, F, F) {
    let t2505 = t2504 * t746;
    let t2508 = t738 * t738;
    let t2509 = F::new(1.0) / t2508;
    let t2510 = t172 * t2509;
    let t2511 = t180 * t180;
    let t2512 = F::new(1.0) / t2511;
    let t2513 = t2369 * t2512;
    let t2516 = -F::cast_from(0.70983522622222222221e-3_f64) * t118 * t2393 * t168 - F::cast_from(0.34246666666666666666e-1_f64) * t268 * t2454 * t732 - F::new(2.0) * t2460 * t2462 + F::new(1.0) * t725 * t2472 + F::cast_from(0.32163958997385070134e2_f64) * t2477 * t2480 + t2426 + t2486 + t2423 - t2408 - t2417 - F::cast_from(0.24415263074675393405e-3_f64) * t118 * t2393 * t181 - F::cast_from(0.10843581300301739842e-1_f64) * t268 * t2490 * t747 - F::cast_from(0.11696447245269292414e1_f64) * t2494 * t2495 + F::cast_from(0.5848223622634646207e0_f64) * t740 * t2505 + F::cast_from(0.17315859105681463759e2_f64) * t2510 * t2513;
    (t2509, t2512, t2516)
}
