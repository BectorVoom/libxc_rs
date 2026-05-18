//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1350/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1350<F: Float>(t62390: F, t67160: F, t67162: F, t67169: F, t69531: F, t69533: F, t69535: F, t69537: F, t69539: F, t69541: F, t69544: F, t69546: F, t69548: F) -> F {
    let t71798 = -t67160 - t67162 + F::new(7.0) / F::new(1152.0) * t69531 - F::new(7.0) / F::new(576.0) * t69533 + F::new(7.0) / F::new(1152.0) * t69535 - F::new(5.0) / F::new(32.0) * t69537 + F::new(5.0) / F::new(96.0) * t69539 + F::new(5.0) / F::new(192.0) * t69541 + t69544 / F::new(8.0) - t69546 / F::new(24.0) - t67169 + t69548 / F::new(192.0) - t62390;
    t71798
}
