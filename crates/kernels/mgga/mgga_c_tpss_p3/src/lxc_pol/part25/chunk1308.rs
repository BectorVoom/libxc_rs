//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1308/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1308<F: Float>(t21060: F, t5570: F, t13719: F, t18454: F, t13715: F, t13736: F, t19476: F, t13707: F, t65607: F, t13711: F, t13741: F, t13745: F) -> (F, F, F, F, F, F, F, F) {
    let t69458 = t21060 * t5570;
    let t69489 = t18454 * t13719;
    let t69491 = t18454 * t13715;
    let t69493 = t19476 * t13736;
    let t69495 = t65607 * t13707;
    let t69497 = t19476 * t13711;
    let t69499 = t18454 * t13741;
    let t69501 = t18454 * t13745;
    (t69458, t69489, t69491, t69493, t69495, t69497, t69499, t69501)
}
