//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1067/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1067<F: Float>(t18442: F, t18473: F, t18535: F, t18569: F, t225: F, t68: F, t484: F, t18215: F, t3440: F, t18211: F, t1653: F, t5012: F, t3578: F, t17691: F, t4972: F, t4582: F) -> (F, F, F, F, F, F, F) {
    let t18571 = t18442 + t18473 + t18535 + t18569;
    let t18572 = t18571 * t225;
    let t18573 = t18572 * t68;
    let t18574 = t18573 * t484;
    let t18577 = t3440 * t18215;
    let t18580 = t3440 * t18211;
    let t18583 = t5012 * t1653;
    let t18584 = t3578 * t18583;
    let t18589 = t4972 * t17691;
    let t18590 = t4582 * t18589;
    (t18571, t18572, t18574, t18577, t18580, t18584, t18590)
}
