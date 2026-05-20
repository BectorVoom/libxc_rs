//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 221/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk221<F: Float>(t1209: F, t478: F, t1017: F, t483: F, t1207: F, t486: F, t61: F, t122: F, t374: F) -> (F, F, F, F, F, F) {
    let t1210 = t1209 * t478;
    let t1211 = t483 * t1017;
    let t1212 = t1210 * t1211;
    let t1213 = t1207 * t1212;
    let t1214 = t61 * t486;
    let t1222 = t374 * t122 * t486;
    (t1210, t1211, t1212, t1213, t1214, t1222)
}
