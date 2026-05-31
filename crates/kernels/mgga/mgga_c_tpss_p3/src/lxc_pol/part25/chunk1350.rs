//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1350/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1350<F: Float>(t62390: F, t67160: F, t67162: F, t67169: F, t69531: F, t69533: F, t69535: F, t69537: F, t69539: F, t69541: F, t69544: F, t69546: F, t69548: F) -> F {
    let t71798 = -t67160 - t67162 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t69531 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t69533 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t69535 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t69537 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t69539 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t69541 + t69544 / F::cast_from(8.0_f64) - t69546 / F::cast_from(24.0_f64) - t67169 + t69548 / F::cast_from(192.0_f64) - t62390;
    t71798
}
