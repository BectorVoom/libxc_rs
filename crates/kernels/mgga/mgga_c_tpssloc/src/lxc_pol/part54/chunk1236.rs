//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1236/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1236<F: Float>(t1888: F, t23270: F, t26729: F, t113038: F, t113045: F, t118924: F, t118928: F, t13053: F, t13463: F, t25168: F, t26582: F, t26713: F, t31343: F, t31409: F, t33405: F, t4268: F, t6627: F, t6632: F, t7516: F, t8553: F, t87013: F, t92981: F) -> (F,) {
    let t121745 = t1888 * t23270 * t26729;
    let t121747 = -6.0 * t87013 * t33405 + t113038 + 2.0 * t6627 * t26582 + 2.0 * t13463 * t8553 + 2.0 * t4268 * t31343 + 2.0 * t4268 * t31409 - t118924 + 2.0 * t26713 * t6632 + 2.0 * t13053 * t8553 - 6.0 * t25168 * t92981 * t7516 - t113045 - 0.49348022005446793095e-1 * t121745 + t118928;
    (t121747,)
}
