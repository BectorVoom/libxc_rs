//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 971/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk971<F: Float>(t31011: F, t3966: F, t8513: F, t32: F, t607: F, t2240: F, t1409: F, t8308: F, t33118: F, t645: F, t46104: F, t8301: F) -> (F, F, F, F, F) {
    let t119928 = t8513 * t31011 * t3966;
    let t119931 = t32 * t607;
    let t119932 = t2240 * t119931;
    let t119933 = t8308 * t1409;
    let t119948 = t8513 * t33118 * t645;
    let t119955 = t46104 * t8301;
    (t119928, t119932, t119933, t119948, t119955)
}
