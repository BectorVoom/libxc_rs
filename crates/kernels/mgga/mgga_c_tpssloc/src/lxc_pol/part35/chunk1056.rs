//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1056/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1056<F: Float>(t4166: F, t6613: F, t1878: F, t23033: F, t253: F, t254: F, t10109: F, t1911: F, t225: F, t7492: F, t1484: F, t857: F, t23204: F, t7488: F, t6562: F, t23168: F, t7480: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25146 = t4166 * t6613;
    let t25154 = t1878 * t23033;
    let t25168 = t253 * t254;
    let t25169 = t10109 * t1911;
    let t25188 = t7492 * t225;
    let t25191 = t857 * t1484;
    let t25205 = t23204 * t7488;
    let t25206 = t6562 * t25205;
    let t25209 = t23168 * t7480;
    (t25146, t25154, t25168, t25169, t25188, t25191, t25205, t25206, t25209)
}
