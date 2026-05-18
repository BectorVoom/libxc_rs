//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1217/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1217<F: Float>(t18750: F, t219: F, t5832: F, t18000: F, t1805: F, t2407: F, t768: F) -> (F, F, F, F) {
    let t18751 = param_beta * t18750;
    let t18753 = t5832 * t219;
    let t18767 = t18000 * t1805 * t2407;
    let t18770 = t768 * t1805;
    (t18751, t18753, t18767, t18770)
}
