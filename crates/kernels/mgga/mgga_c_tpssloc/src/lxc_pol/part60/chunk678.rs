//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 678/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk678<F: Float>(t1509: F, t1519: F, t252: F, t5584: F, t120: F, t5611: F, t225: F, t5559: F, t5632: F, t5561: F, t6151: F, t6153: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16758 = t1519 * t1509;
    let t16815 = t252 * t5584;
    let t16839 = t120 * t5584;
    let t16891 = t120 * t5611;
    let t17030 = t252 * t5611;
    let t17052 = t5559 * t225;
    let t17090 = t5632 * t225;
    let t17092 = t5561 * t225;
    let t19232 = t6151 * t225;
    let t19234 = t6153 * t225;
    (t16758, t16815, t16839, t16891, t17030, t17052, t17090, t17092, t19232, t19234)
}
