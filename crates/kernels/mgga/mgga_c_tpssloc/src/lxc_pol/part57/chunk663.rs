//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 663/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk663<F: Float>(t23168: F, t7521: F, t22893: F, t7520: F, t23164: F, t1519: F, t234: F, t23204: F, t7479: F, t225: F, t7511: F, t2752: F, t7540: F, t10143: F, t25: F, t1625: F, t6703: F) -> (F, F, F, F, F, F, F, F) {
    let t25310 = t23168 * t7521;
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    let t25345 = t23204 * t7479;
    let t25346 = t23164 * t25345;
    let t25348 = t7511 * t225;
    let t25358 = t7540 * t2752;
    let t25373 = t10143 * t25;
    let t25406 = t6703 * t1625;
    (t25310, t25317, t25319, t25346, t25348, t25358, t25373, t25406)
}
