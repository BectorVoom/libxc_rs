//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3113/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3113<F: Float>(t300: F, t63457: F, t63506: F, t63561: F, t63611: F, t63715: F, t63760: F, t64260: F, t64442: F, t1254: F, t5091: F, t11282: F, t6084: F) -> (F, F, F) {
    let t64446 = t300 * (t63457 + t63506 + t63561 + t63611 + t63715 + t63760 + t64260 + t64442);
    let t64447 = t1254 * t5091;
    let t64451 = t11282 * t6084;
    (t64446, t64447, t64451)
}
