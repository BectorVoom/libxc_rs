//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 796/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk796<F: Float>(t205: F, t9558: F, t210: F, t214: F, t9458: F, t213: F, t776: F, t221: F, t2553: F, t59: F, t8705: F) -> (F, F, F, F) {
    let t9559 = t205 * t9558;
    let t9561 = t210 * t214 * t9458;
    let t9564 = t213 * t776;
    let t9566 = t221 * t9564 * t2553;
    let t9569 = t59 * t8705;
    (t9559, t9561, t9566, t9569)
}
