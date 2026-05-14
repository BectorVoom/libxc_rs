//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1040/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1040<F: Float>(t14279: F, t14373: F, t14428: F, t14469: F, t300: F, t4446: F, t961: F, t2948: F, t4483: F, t14364: F, t2907: F, t4496: F, t959: F, t2952: F, t10623: F, t1589: F) -> (F, F, F, F, F, F, F) {
    let t14472 = t300 * (t14279 + t14373 + t14428 + t14469);
    let t14473 = t300 * t4446;
    let t14475 = 0.11696447245269292414e1 * t14473 * t961;
    let t14477 = 0.5848223622634646207e0 * t4483 * t2948;
    let t14479 = 0.19751673498613801407e-1 * t300 * t14364;
    let t14480 = t4496 * t2907;
    let t14482 = 0.35089341735807877242e1 * t959 * t14480;
    let t14484 = 0.17315859105681463759e2 * t4483 * t2952;
    let t14486 = 0.5848223622634646207e0 * t10623 * t1589;
    (t14472, t14475, t14477, t14479, t14482, t14484, t14486)
}
