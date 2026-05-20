//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2092/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2092<F: Float>(t9546: F, t9555: F, t2573: F, t41008: F, t2566: F, t2570: F, t9551: F, t2588: F, t40341: F, t12998: F, t2553: F, t686: F, t9524: F) -> (F, F, F, F, F) {
    let t41192 = t9546 * t9555;
    let t41194 = t41008 * t2573;
    let t41196 = t2566 * t2570;
    let t41197 = t41196 * t9551;
    let t41200 = F::cast_from(0.99537037037037037035e-1_f64) * t40341 * t2588;
    let t41203 = t12998 * t686 * t9524 * t2553;
    (t41192, t41194, t41197, t41200, t41203)
}
