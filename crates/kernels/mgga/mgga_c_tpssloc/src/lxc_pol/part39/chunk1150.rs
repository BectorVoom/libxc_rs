//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1150/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1150<F: Float>(t14480: F, t959: F, t2952: F, t4483: F, t10623: F, t1589: F, t14257: F, t14262: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14424: F, t14472: F, t14475: F, t14477: F, t14479: F) -> (F, F, F, F) {
    let t14482 = F::new(0.35089341735807877242e1) * t959 * t14480;
    let t14484 = F::new(0.17315859105681463759e2) * t4483 * t2952;
    let t14486 = F::new(0.5848223622634646207e0) * t10623 * t1589;
    let t14487 = -t14257 - t14262 + t14472 - t14376 + t14378 - t14381 - t14384 - t14387 + t14391 + t14394 + t14398 + t14424 - t14475 - t14477 + t14479 - t14482 - t14484 - t14486;
    (t14482, t14484, t14486, t14487)
}
