//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1207/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1207<F: Float>(t18397: F, t65452: F, t13215: F, t5527: F, t1270: F, t12810: F, t18546: F, t6242: F, t4466: F, t60738: F, t12865: F, t18454: F, t12819: F, t12831: F, t19476: F, t13000: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65453 = t18397 * t65452;
    let t65455 = t5527 * t13215;
    let t65501 = t1270 * t12810;
    let t65533 = t6242 * t18546;
    let t65551 = t60738 * t4466;
    let t65553 = t18454 * t12865;
    let t65555 = t18454 * t12819;
    let t65557 = t19476 * t12831;
    let t65559 = t19476 * t13000;
    (t65453, t65455, t65501, t65533, t65551, t65553, t65555, t65557, t65559)
}
