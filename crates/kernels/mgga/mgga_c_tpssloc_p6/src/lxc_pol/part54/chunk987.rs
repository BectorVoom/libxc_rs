//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 987/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk987<F: Float>(t7537: F, t865: F, t2718: F, t23204: F, t7488: F, t6562: F, t23168: F, t7480: F, t6547: F, t7489: F, t23237: F, t1880: F) -> (F, F, F, F, F, F) {
    let t25199 = t7537 * t865;
    let t25200 = t2718 * t25199;
    let t25205 = t23204 * t7488;
    let t25206 = t6562 * t25205;
    let t25209 = t23168 * t7480;
    let t25211 = t6547 * t7489;
    let t25213 = t23237 * t7488;
    let t25214 = t1880 * t25213;
    (t25199, t25200, t25206, t25209, t25211, t25214)
}
