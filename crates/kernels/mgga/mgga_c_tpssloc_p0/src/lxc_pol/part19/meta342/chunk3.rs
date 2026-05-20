//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1222/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1222<F: Float>(t2588: F, t40341: F, t12998: F, t2553: F, t686: F, t9524: F, t13012: F, t9566: F, t207: F, t215: F, t39933: F, t40344: F, t795: F) -> (F, F, F, F, F) {
    let t41200 = F::cast_from(0.99537037037037037035e-1_f64) * t40341 * t2588;
    let t41203 = t12998 * t686 * t9524 * t2553;
    let t41205 = t13012 * t9566;
    let t41209 = F::cast_from(0.14979423868312757201e0_f64) * t39933 * t207 * t215;
    let t41212 = F::cast_from(0.11265432098765432099e0_f64) * t40344 * t207 * t795;
    (t41200, t41203, t41205, t41209, t41212)
}
