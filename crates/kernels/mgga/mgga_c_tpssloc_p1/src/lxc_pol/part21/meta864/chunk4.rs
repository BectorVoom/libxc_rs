//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3155/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3155<F: Float>(t63731: F, t63733: F, t63735: F, t63737: F, t63739: F, t63741: F, t63743: F, t63745: F, t63747: F, t63752: F, t63754: F, t63757: F, t63759: F) -> F {
    let t65286 = -t63731 - t63733 - t63735 - t63737 + t63739 + t63741 + t63743 + t63745 + t63747 + t63752 + t63754 + t63757 + t63759;
    t65286
}
