//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 407/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk407<F: Float>(t471: F, t5023: F, t1193: F, t1706: F, t135: F, t1725: F, t1174: F, t1752: F, t225: F, t1243: F, t5000: F, t1390: F, t1845: F, t172: F, t1787: F, t763: F) -> (F, F, F, F, F, F, F, F) {
    let t5024 = t471 * t5023;
    let t5036 = t1706 * t1193;
    let t5040 = t135 * t1725;
    let t5041 = t1174 * t5040;
    let t5055 = t1752 * t225;
    let t5064 = t5000 * t1243;
    let t5122 = t1845 * t1390;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    (t5024, t5036, t5040, t5041, t5055, t5064, t5122, t5155)
}
