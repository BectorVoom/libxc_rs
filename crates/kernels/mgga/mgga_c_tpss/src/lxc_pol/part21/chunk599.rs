//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 599/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk599<F: Float>(t2525: F, t866: F, t846: F, t844: F, t269: F) -> (F, F, F, F, F) {
    let t2526 = t2525 * t866;
    let t2528 = 1.0 * t846 * t2526;
    let t2529 = t844 * t844;
    let t2530 = 1.0 / t2529;
    let t2531 = t269 * t2530;
    (t2526, t2528, t2529, t2530, t2531)
}
