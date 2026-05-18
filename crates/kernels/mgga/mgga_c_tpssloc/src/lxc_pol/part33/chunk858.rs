//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 858/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk858<F: Float>(t1553: F, t2403: F, t1543: F, t2791: F, t1597: F, t4509: F, t10189: F, t10224: F, t1592: F, t973: F, t1599: F, t698: F) -> (F, F, F, F, F, F) {
    let t13642 = t2403 * t1553;
    let t13727 = t1543 * t2791;
    let t13769 = t4509 * t1597;
    let t13847 = t10189 * t1597;
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    let t13908 = t698 * t1599;
    (t13642, t13727, t13769, t13847, t13896, t13908)
}
