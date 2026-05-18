//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1098/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1098<F: Float>(t24601: F, t27437: F, t24590: F, t8002: F, t3247: F, t497: F, t3961: F, t24574: F, t8067: F, t1184: F, t1715: F, t24745: F, t7363: F) -> (F, F, F, F, F, F) {
    let t27438 = t24601 * t27437;
    let t27441 = t24590 * t8002;
    let t27444 = t497 * t3247;
    let t27445 = t27444 * t3961;
    let t27446 = t24601 * t27445;
    let t27451 = t24574 * t8067;
    let t27453 = t1715 * t1184;
    let t27454 = t24745 * t7363;
    (t27438, t27441, t27446, t27451, t27453, t27454)
}
