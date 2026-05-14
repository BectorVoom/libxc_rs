//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1082/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1082<F: Float>(t32: F, t607: F, t2240: F, t1409: F, t8308: F, t33118: F, t645: F, t8513: F, t46104: F, t8301: F, t1433: F, t641: F, t4017: F, t79: F, t4021: F, t8307: F) -> (F, F, F, F, F, F, F) {
    let t119931 = t32 * t607;
    let t119932 = t2240 * t119931;
    let t119933 = t8308 * t1409;
    let t119948 = t8513 * t33118 * t645;
    let t119955 = t46104 * t8301;
    let t119971 = t8513 * t641 * t1433;
    let t119975 = t8513 * t79 * t4017;
    let t119990 = t8513 * t8307 * t4021;
    (t119932, t119933, t119948, t119955, t119971, t119975, t119990)
}
