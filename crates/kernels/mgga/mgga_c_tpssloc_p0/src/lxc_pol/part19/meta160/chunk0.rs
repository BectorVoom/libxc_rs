//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 777/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk777<F: Float>(t2359: F, t626: F, t655: F, t2332: F, t666: F, t2331: F, t2358: F, t2261: F, t93: F, t94: F, t2342: F, t659: F, tau0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9363 = t626 * t2359;
    let t9364 = t655 * t655;
    let t9365 = F::new(1.0) / t9364;
    let t9366 = t2332 * t666;
    let t9367 = t9365 * t9366;
    let t9370 = t2331 * t666;
    let t9371 = t9370 * t2358;
    let t9374 = tau0 * t2261;
    let t9383 = t94 * t93;
    let t9384 = F::new(1.0) / t9383;
    let t9385 = t2342 * t659;
    (t9363, t9364, t9365, t9367, t9370, t9371, t9374, t9384, t9385)
}
