//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 947/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk947<F: Float>(t70330: F, t71789: F, t71802: F, t76064: F, t76075: F, t76084: F, t78473: F, t78474: F, t78475: F, t78476: F, t78477: F, t78478: F, t78479: F, t78480: F, t78482: F, t78483: F, t78484: F) -> (F,) {
    let t80512 = -t71789 - 0.40878380883436523435e-5 * t70330 - t78473 - t78474 - t78475 - t76064 + t78476 - t78477 - t78478 + t78479 - t78480 + t71802 - t78482 + t76075 + t78483 + t78484 - t76084;
    (t80512,)
}
