//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2885/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2885<F: Float>(t13615: F, t4378: F, t17223: F, t2807: F, t2799: F, t41935: F, t5698: F, t10595: F, t5705: F, t41942: F, t10599: F, t17271: F, t2798: F, t896: F) -> (F, F, F, F, F, F, F) {
    let t60243 = t4378 * t13615;
    let t60245 = t17223 * t2807;
    let t60248 = t41935 * t5698 * t2799;
    let t60251 = t10595 * t5705 * t2799;
    let t60254 = t41942 * t5698 * t2799;
    let t60257 = t10599 * t5705 * t2799;
    let t60260 = t2798 * t17271 * t896;
    (t60243, t60245, t60248, t60251, t60254, t60257, t60260)
}
