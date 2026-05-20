//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1824/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1824<F: Float>(t1268: F, t26135: F, t12725: F, t1874: F, t510: F, t652: F, t7000: F, t7685: F, t6876: F, t7688: F, t6999: F, t7753: F) -> (F, F, F, F, F, F, F) {
    let t26137 = F::new(2.0) * t1268 * t26135;
    let t26141 = F::new(2.0) * t12725 * t1874;
    let t26142 = t510 * t26135;
    let t26144 = F::new(2.0) * t652 * t26142;
    let t26145 = t7685 * t7000;
    let t26147 = F::new(3.0) * t6876 * t7688;
    let t26149 = t7753 * t6999;
    (t26137, t26141, t26142, t26144, t26145, t26147, t26149)
}
