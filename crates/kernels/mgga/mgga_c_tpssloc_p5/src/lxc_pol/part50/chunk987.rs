//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 987/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk987<F: Float>(t12725: F, t1874: F, t26135: F, t510: F, t652: F, t7000: F, t7685: F, t6876: F, t7688: F, t6999: F, t7753: F, t1983: F) -> (F, F, F, F, F, F, F) {
    let t26141 = F::cast_from(2.0_f64) * t12725 * t1874;
    let t26142 = t510 * t26135;
    let t26144 = F::cast_from(2.0_f64) * t652 * t26142;
    let t26145 = t7685 * t7000;
    let t26147 = F::cast_from(3.0_f64) * t6876 * t7688;
    let t26149 = t7753 * t6999;
    let t26150 = t1983 * t26149;
    (t26141, t26142, t26144, t26145, t26147, t26149, t26150)
}
