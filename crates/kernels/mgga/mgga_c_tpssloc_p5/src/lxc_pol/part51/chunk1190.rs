//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1190/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1190<F: Float>(t112: F, t31699: F, t1873: F, t23938: F, t26977: F, t6534: F, t7042: F, t2039: F, t31537: F, t88: F, t7056: F, t8601: F) -> (F, F, F, F, F, F, F, F) {
    let t31700 = t31699 * t112;
    let t31704 = F::new(2.0) * t23938 * t1873;
    let t31706 = F::new(2.0) * t26977 * t1873;
    let t31708 = F::new(2.0) * t7042 * t6534;
    let t31716 = F::new(2.0) * t31537 * t2039;
    let t31717 = t88 * t6534;
    let t31719 = F::new(2.0) * t31717 * t2039;
    let t31721 = F::new(2.0) * t8601 * t7056;
    (t31700, t31704, t31706, t31708, t31716, t31717, t31719, t31721)
}
