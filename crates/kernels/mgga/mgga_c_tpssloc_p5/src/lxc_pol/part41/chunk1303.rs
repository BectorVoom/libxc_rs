//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1303/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1303<F: Float>(t1851: F, t8299: F, t30581: F, t580: F, t2212: F, t6470: F, t110919: F, t111289: F, t111291: F, t111293: F, t111842: F, t112062: F, t1396: F, t1398: F, t1858: F, t20149: F, t30350: F, t30616: F, t6471: F, t6483: F, t8200: F, t8217: F) -> F {
    let t112065 = t1851 * t8299;
    let t112073 = t30581 * t580;
    let t112074 = t6470 * t2212;
    let t112075 = t1398 * (t111842 + t112062) + F::new(2.0) * t112065 + t1396 * t30616 + t110919 + t20149 * t2212 + t111289 + t8200 * t6483 + t111291 + F::new(2.0) * t30350 * t1858 + t6471 * t8217 + t112073 + t112074 + t111293;
    t112075
}
