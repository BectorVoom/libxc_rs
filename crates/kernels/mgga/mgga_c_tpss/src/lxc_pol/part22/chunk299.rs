//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 299/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk299<F: Float>(t339: F, t349: F, t956: F, t127: F, t359: F, t361: F, t355: F, t353: F, t357: F) -> (F, F, F, F) {
    let t958 = t339 * t349 * t956;
    let t962 = t359 * t127 * t361;
    let t964 = t355 * t962 / 4608.0;
    let t965 = t353 * t357;
    let t967 = t339 * t349 * t965;
    (t958, t962, t964, t967)
}
