//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 838/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk838<F: Float>(t3403: F, t6105: F, t1164: F, t338: F, t5416: F, t3441: F, t5392: F, t3440: F, t4904: F, t4919: F, t3455: F, t1177: F) -> (F, F, F, F, F, F, F, F) {
    let t6106 = t6105 * t3403;
    let t6108 = F::new(0.17315859105681463759e2) * t1164 * t6106;
    let t6109 = t5416 * t338;
    let t6119 = t3441 * t5392;
    let t6120 = t3440 * t6119;
    let t6123 = t4919 * t4904;
    let t6126 = t3455 * t5392;
    let t6127 = t1177 * t6126;
    (t6106, t6108, t6109, t6119, t6120, t6123, t6126, t6127)
}
