//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1234/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1234<F: Float>(t65564: F, t65567: F, t67143: F, t67150: F, t69510: F, t69512: F, t69515: F, t69517: F, t69519: F, t69521: F, t69523: F, t69525: F, t69527: F, t62390: F, t67160: F, t67162: F, t67169: F, t69531: F, t69533: F, t69535: F, t69537: F, t69539: F, t69541: F, t69544: F, t69546: F, t69548: F) -> (F, F) {
    let t71787 = t69510 / 96.0 + t69512 / 96.0 - t67143 - t65564 + t69515 / 192.0 - 7.0 / 144.0 * t69517 + t69519 / 384.0 + t69521 / 192.0 - t69523 / 384.0 - 35.0 / 54.0 * t65567 - 7.0 / 24.0 * t69525 + 7.0 / 72.0 * t69527 + t67150;
    let t71798 = -t67160 - t67162 + 7.0 / 1152.0 * t69531 - 7.0 / 576.0 * t69533 + 7.0 / 1152.0 * t69535 - 5.0 / 32.0 * t69537 + 5.0 / 96.0 * t69539 + 5.0 / 192.0 * t69541 + t69544 / 8.0 - t69546 / 24.0 - t67169 + t69548 / 192.0 - t62390;
    (t71787, t71798)
}
