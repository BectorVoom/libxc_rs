//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1148/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1148<F: Float>(t120687: F, t120691: F, t120697: F, t120699: F, t120702: F, t120708: F, t120719: F, t120721: F, t120728: F, t120730: F, t120735: F, t123228: F, t123229: F, t123235: F, t123244: F, t125910: F, t1459: F, t32609: F, t4026: F, t4037: F, t4073: F, t8913: F) -> (F,) {
    let t125963 = -2.0 * t125910 * t1459 - 2.0 * t32609 * t4037 - 2.0 * t32609 * t4073 - t4026 * t8913 - t120687 - t120691 + t120697 + t120699 + t120702 - t120708 - t120719 - t120721 - t120728 - t120730 - t120735 - 2.0 * t123228 - 6.0 * t123229 + 2.0 * t123235 - 4.0 * t123244;
    (t125963,)
}
