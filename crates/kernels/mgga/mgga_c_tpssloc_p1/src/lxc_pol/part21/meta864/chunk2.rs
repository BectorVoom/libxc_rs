//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3153/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3153<F: Float>(t63560: F, t63563: F, t63566: F, t63568: F, t63571: F, t63574: F, t63576: F, t63579: F, t63582: F, t63585: F, t63587: F, t63591: F, t63594: F) -> F {
    let t65282 = t63560 - t63563 - t63566 - t63568 - t63571 - t63574 - t63576 - t63579 - t63582 - t63585 + t63587 + t63591 + t63594;
    t65282
}
