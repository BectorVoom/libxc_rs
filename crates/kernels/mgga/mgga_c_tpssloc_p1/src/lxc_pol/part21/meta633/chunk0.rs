//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2417/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2417<F: Float>(t2578: F, t41189: F, t9546: F, t9555: F, t2573: F, t41008: F, t2566: F, t2570: F, t9551: F, t2588: F, t40341: F, t207: F, t215: F, t39933: F) -> (F, F, F, F, F, F, F) {
    let t41190 = t41189 * t2578;
    let t41192 = t9546 * t9555;
    let t41194 = t41008 * t2573;
    let t41196 = t2566 * t2570;
    let t41197 = t41196 * t9551;
    let t41200 = F::cast_from(0.99537037037037037035e-1_f64) * t40341 * t2588;
    let t41209 = F::cast_from(0.14979423868312757201e0_f64) * t39933 * t207 * t215;
    (t41190, t41192, t41194, t41196, t41197, t41200, t41209)
}
