//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2412/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2412<F: Float>(t2606: F, t41008: F, t782: F, t9558: F, t2617: F, t9600: F, t849: F, t2642: F, t9612: F, t786: F, t9569: F, t805: F) -> (F, F, F, F, F, F, F) {
    let t41009 = t41008 * t2606;
    let t41011 = t782 * t9558;
    let t41052 = t2617 * t9600;
    let t41053 = t41052 * t849;
    let t41063 = t9612 * t2642;
    let t41083 = t9569 * t786;
    let t41084 = t41083 * t805;
    (t41009, t41011, t41052, t41053, t41063, t41083, t41084)
}
