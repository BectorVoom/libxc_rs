//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2319/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2319<F: Float>(t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40685: F, t40708: F, t40714: F, t40716: F, t46207: F, t67097: F, t67100: F, t67104: F, t67105: F, t67127: F, t67132: F, t67133: F) -> F {
    let t67449 = t67097 - t40685 - t67100 + t67104 + t67105 + t39373 - t39397 - t39400 + t40708 + t39408 + t39411 + t46207 + t67127 - t40714 + t40716 + t67132 - t67133;
    t67449
}
