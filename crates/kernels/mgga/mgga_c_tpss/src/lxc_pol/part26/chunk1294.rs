//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1294/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1294<F: Float>(t13736: F, t19476: F, t13707: F, t65607: F, t13711: F, t13741: F, t18454: F, t13745: F, t13760: F, t13695: F, t13682: F, t60685: F, t65552: F, t69489: F, t69491: F, t13749: F) -> (F, F) {
    let t69493 = t19476 * t13736;
    let t69495 = t65607 * t13707;
    let t69497 = t19476 * t13711;
    let t69499 = t18454 * t13741;
    let t69501 = t18454 * t13745;
    let t69503 = t19476 * t13760;
    let t69505 = t18454 * t13695;
    let t69507 = t18454 * t13682;
    let t69509 = t65552 + t69489 / 192.0 - 5.0 / 192.0 * t69491 - t60685 - t69493 / 96.0 - t69495 / 256.0 + t69497 / 256.0 + t69499 / 384.0 - t69501 / 1536.0 - t69503 / 192.0 - t69505 / 1536.0 - 5.0 / 384.0 * t69507;
    let t69510 = t18454 * t13749;
    (t69509, t69510)
}
