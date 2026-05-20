//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 750/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk750<F: Float>(t72: F, t7431: F, t1410: F, t605: F, t1409: F, t6500: F, t6503: F) -> (F, F, F) {
    let t7432 = t72 * t7431;
    let t7435 = t605 * t1410;
    let t7440 = F::new(5.0) / F::new(6.0) * t6500 * t1409 + t6503;
    (t7432, t7435, t7440)
}
