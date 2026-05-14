//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 592/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk592<F: Float>(t2973: F, t425: F, t2834: F, t2891: F, t1071: F) -> (F, F, F, F, F) {
    let t2974 = t425 * t2973;
    let t2981 = 0.40256666666666666667e0 * t2834;
    let t2988 = 0.137975e0 * t2891;
    let t2997 = t1071 * t1071;
    let t2998 = 1.0 / t2997;
    (t2974, t2981, t2988, t2997, t2998)
}
