//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1167/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1167<F: Float>(t39476: F, t39479: F, t39483: F, t39490: F, t39492: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39511: F, t39513: F, t39515: F) -> F {
    let t40214 = -t39476 - t39479 + t39483 - t39490 + t39492 - t39496 + t39499 + t39502 - t39505 - t39508 + t39511 + t39513 - t39515;
    t40214
}
