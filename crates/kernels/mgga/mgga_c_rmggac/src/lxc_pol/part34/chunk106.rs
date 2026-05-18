//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 106/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk106<F: Float>(t155: F, t422: F, t181: F, t388: F, t156: F, t2: F, t180: F, t243: F, t245: F, t171: F, t410: F, t416: F, t417: F) -> (F, F, F, F, F, F, F) {
    let t423 = t155 * t422;
    let t425 = F::new(0.19751673498613801407e-1) * t388 * t181;
    let t426 = t156 * t2;
    let t428 = t243 * t245 * t180;
    let t430 = F::new(0.18311447306006545054e-3) * t426 * t428;
    let t431 = t156 * t171;
    let t433 = t410 * t416 * t417;
    (t423, t425, t426, t428, t430, t431, t433)
}
