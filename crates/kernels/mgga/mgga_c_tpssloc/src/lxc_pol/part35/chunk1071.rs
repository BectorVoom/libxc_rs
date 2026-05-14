//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1071/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1071<F: Float>(t1751: F, t7284: F, t3247: F, t497: F, t24574: F, t8067: F, t477: F, t3502: F, t491: F, t24813: F, t1209: F, t1419: F, t6794: F, t131: F, t467: F, t225: F, t8034: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27426 = t7284 * t1751;
    let t27444 = t497 * t3247;
    let t27451 = t24574 * t8067;
    let t27460 = t477 * t1751;
    let t27488 = t3502 * t491;
    let t27489 = t24813 * t27488;
    let t27495 = t1209 * t491;
    let t27496 = t24813 * t27495;
    let t27505 = t1419 * t6794;
    let t27506 = t27505 * t131;
    let t27507 = t27506 * t467;
    let t27516 = t8034 * t225;
    (t27426, t27444, t27451, t27460, t27489, t27495, t27496, t27505, t27506, t27507, t27516)
}
