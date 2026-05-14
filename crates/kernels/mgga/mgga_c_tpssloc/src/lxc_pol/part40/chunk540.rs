//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 540/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk540<F: Float>(t2617: F, t816: F, t809: F, t838: F, t842: F, t233: F, t813: F, t236: F, t240: F, t812: F, t232: F) -> (F, F, F, F, F, F, F, F) {
    let t2618 = t2617 * t816;
    let t2621 = t809 * t838;
    let t2623 = t2617 * t842;
    let t2627 = 1.0 / t813 / t233;
    let t2628 = t2627 * t236;
    let t2629 = t2628 * t240;
    let t2630 = t812 * t2629;
    let t2632 = t232 * t232;
    (t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2632)
}
