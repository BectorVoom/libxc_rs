//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 630/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk630<F: Float>(t837: F, t949: F, t2741: F, t357: F, t956: F, t339: F, t349: F) -> (F, F, F, F) {
    let t2742 = t949 * t837;
    let t2743 = t2741 * t2742;
    let t2746 = t956 * t357;
    let t2748 = t339 * t349 * t2746;
    (t2742, t2743, t2746, t2748)
}
