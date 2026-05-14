//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 397/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk397<F: Float>(t4166: F, t816: F, t1500: F, t838: F, t842: F, t244: F, t67: F, t246: F, t120: F, t1509: F, t1512: F, t2639: F, t1474: F, t172: F, t763: F, t1471: F, t706: F) -> (F, F, F, F, F, F, F, F) {
    let t4167 = t4166 * t816;
    let t4170 = t1500 * t838;
    let t4172 = t4166 * t842;
    let t4179 = t244 * t67;
    let t4180 = t4179 * t246;
    let t4181 = t120 * t1509;
    let t4187 = t2639 * t1512;
    let t4199 = t1474 * t172;
    let t4200 = t4199 * t763;
    let t4205 = t706 * t1471;
    (t4167, t4170, t4172, t4180, t4181, t4187, t4200, t4205)
}
