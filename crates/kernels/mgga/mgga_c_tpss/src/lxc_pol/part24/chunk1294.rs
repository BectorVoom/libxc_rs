//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1294/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1294<F: Float>(t60723: F, t65593: F, t65601: F, t65617: F, t69531: F, t69533: F, t69535: F, t69537: F, t69539: F, t69541: F, t69544: F, t69546: F, t69548: F, t13853: F, t5721: F, t18464: F, t5420: F) -> (F, F, F) {
    let t69550 = -t65593 - t65601 + 7.0 / 2304.0 * t69531 - 7.0 / 1152.0 * t69533 + 7.0 / 2304.0 * t69535 - 5.0 / 64.0 * t69537 + 5.0 / 192.0 * t69539 + 5.0 / 384.0 * t69541 + t69544 / 16.0 - t69546 / 48.0 - t65617 + t69548 / 384.0 - t60723;
    let t69551 = t5721 * t13853;
    let t69553 = t18464 * t5420;
    (t69550, t69551, t69553)
}
