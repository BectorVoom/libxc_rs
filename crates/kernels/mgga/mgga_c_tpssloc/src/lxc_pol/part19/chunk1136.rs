//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1136/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1136<F: Float>(t39373: F, t39397: F, t39400: F, t40677: F, t40679: F, t40681: F, t40683: F, t40685: F, t40688: F, t40690: F, t40708: F, t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t40711: F, t40714: F, t40716: F, t40721: F, t40723: F) -> (F, F) {
    let t41242 = t40677 - t40679 + t40681 + t40683 - t40685 + t40688 + t40690 + t39373 - t39397 - t39400 + t40708;
    let t41244 = t39408 + t39411 + t40711 - t40714 + t40716 + t39463 - t39468 - t40721 - t40723 - t39472 - t39476;
    (t41242, t41244)
}
