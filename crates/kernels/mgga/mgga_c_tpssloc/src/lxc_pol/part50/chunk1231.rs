//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1231/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1231<F: Float>(t7440: F, t79: F, t641: F, t8513: F, t33118: F, t645: F, t6504: F, t46104: F, t8301: F, t26043: F, t8307: F, t1433: F) -> (F, F, F, F, F, F) {
    let t119942 = t79 * t7440;
    let t119944 = t8513 * t119942 * t641;
    let t119948 = t8513 * t33118 * t645;
    let t119952 = t8513 * t33118 * t6504;
    let t119955 = t46104 * t8301;
    let t119965 = t8513 * t8307 * t26043;
    let t119971 = t8513 * t641 * t1433;
    (t119944, t119948, t119952, t119955, t119965, t119971)
}
