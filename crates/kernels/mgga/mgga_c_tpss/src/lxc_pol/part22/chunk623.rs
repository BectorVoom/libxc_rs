//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 623/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk623<F: Float>(t1043: F, t2905: F, t1024: F, t1022: F, t394: F) -> (F, F, F, F, F) {
    let t2906 = t2905 * t1043;
    let t2908 = 1.0 * t1024 * t2906;
    let t2909 = t1022 * t1022;
    let t2910 = 1.0 / t2909;
    let t2911 = t394 * t2910;
    (t2906, t2908, t2909, t2910, t2911)
}
