//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1191/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1191<F: Float>(t32331: F, t607: F, t8308: F, t63: F, t79: F, t641: F, t8513: F, t625: F, t8307: F) -> (F, F, F, F, F) {
    let t32332 = t32331 * t607;
    let t32333 = t8308 * t32332;
    let t32338 = t79 * t63;
    let t32340 = t8513 * t32338 * t641;
    let t32343 = t8307 * t625;
    let t32344 = t8513 * t32343;
    (t32333, t32338, t32340, t32343, t32344)
}
