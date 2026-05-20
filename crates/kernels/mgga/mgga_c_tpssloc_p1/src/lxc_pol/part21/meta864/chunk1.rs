//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3152/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3152<F: Float>(t63446: F, t63449: F, t63451: F, t63557: F, t64514: F, t64517: F, t64520: F, t64522: F, t64524: F, t64528: F, t64530: F, t64533: F) -> F {
    let t65281 = t63446 - t63449 + t63451 + t64514 - t64517 - t64520 + t64522 - t64524 - t64528 + t64530 - t64533 + t63557;
    t65281
}
