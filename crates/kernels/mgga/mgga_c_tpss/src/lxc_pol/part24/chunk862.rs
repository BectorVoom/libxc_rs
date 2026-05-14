//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 862/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk862<F: Float>(t6171: F, t935: F, t1482: F, t1726: F, t5632: F, t1464: F, t342: F, t345: F) -> (F, F, F, F) {
    let t6172 = t6171 * t935;
    let t6174 = t1726 * t1482;
    let t6175 = t5632 * t6174;
    let t6179 = t1464 * t342 * t345;
    (t6172, t6174, t6175, t6179)
}
