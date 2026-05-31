//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2361/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2361<F: Float>(t225: F, t48427: F, t3082: F, t4622: F, t1040: F, t13941: F, t10231: F, t13555: F, t973: F, t1036: F, t13751: F, t10422: F, t14229: F, t3070: F) -> (F, F, F, F, F, F) {
    let t48428 = t48427 * t225;
    let t48430 = t4622 * t3082;
    let t48431 = t48430 / F::cast_from(864.0_f64);
    let t48432 = t13941 * t1040;
    let t48441 = t973 * t10231 * t13555;
    let t48446 = t13751 * t1036;
    let t48460 = t3070 * t10422 * t14229;
    (t48428, t48431, t48432, t48441, t48446, t48460)
}
