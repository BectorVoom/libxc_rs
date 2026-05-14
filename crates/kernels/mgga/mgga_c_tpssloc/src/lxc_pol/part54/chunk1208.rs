//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1208/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1208<F: Float>(t115925: F, t25989: F, t22574: F, t32193: F, t33357: F, t27219: F, t8526: F, t25988: F, t36740: F, t26168: F, t8607: F, t31747: F, t4028: F, t26149: F, t26161: F, t33221: F, t92200: F) -> (F, F, F, F, F, F, F, F) {
    let t121162 = 3.0 * t115925 * t25989;
    let t121165 = 3.0 * t22574 * t32193 * t33357;
    let t121169 = 2.0 * t8526 * t27219;
    let t121174 = 3.0 * t22574 * t36740 * t25988;
    let t121177 = 3.0 * t8607 * t26168;
    let t121179 = 2.0 * t4028 * t31747;
    let t121181 = t8607 * t26149;
    let t121184 = 2.0 * t26161 * t92200 * t33221;
    (t121162, t121165, t121169, t121174, t121177, t121179, t121181, t121184)
}
