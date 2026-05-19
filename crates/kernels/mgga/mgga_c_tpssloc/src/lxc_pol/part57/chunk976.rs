//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 976/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk976<F: Float>(t1985: F, t28205: F, t31137: F, t120632: F, t22633: F, t22635: F, t31099: F, t6347: F, t26331: F, t6330: F, t6287: F, t652: F, t8326: F) -> (F, F, F, F, F) {
    let t127448 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t31137 * t28205;
    let t127455 = F::cast_from(0.76763589786250567036e-1_f64) * t120632;
    let t127459 = F::cast_from(0.3289868133696452873e-1_f64) * t22633 * t22635 * t31099 * t6347;
    let t127463 = F::cast_from(0.9869604401089358619e-1_f64) * t26331 * t22635 * t31099 * t6330;
    let t127539 = F::new(2.0) * t652 * t6287 * t8326;
    (t127448, t127455, t127459, t127463, t127539)
}
