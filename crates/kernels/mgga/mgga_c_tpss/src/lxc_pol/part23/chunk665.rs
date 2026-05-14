//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 665/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk665<F: Float>(t3053: F, t450: F, t1112: F, t242: F, t359: F, t461: F, t651: F, t460: F, t1127: F, t126: F) -> (F, F, F, F, F) {
    let t3081 = t3053 * t450;
    let t3082 = t1112 * t3081;
    let t3083 = t242 * t3082;
    let t3087 = t359 * t651 * t461;
    let t3089 = t460 * t3087 / 13824.0;
    let t3090 = t126 * t1127;
    (t3081, t3083, t3087, t3089, t3090)
}
