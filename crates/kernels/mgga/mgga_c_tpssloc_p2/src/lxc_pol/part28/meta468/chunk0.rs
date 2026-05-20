//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1677/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1677<F: Float>(t25154: F, t25155: F, t253: F, t254: F, t1484: F, t857: F, t865: F, t23270: F, t22986: F, t23204: F, t7488: F, t6562: F) -> (F, F, F, F, F, F, F, F) {
    let t25156 = t25154 * t25155;
    let t25168 = t253 * t254;
    let t25191 = t857 * t1484;
    let t25192 = t25191 * t865;
    let t25193 = t23270 * t25192;
    let t25194 = t22986 * t25193;
    let t25205 = t23204 * t7488;
    let t25206 = t6562 * t25205;
    (t25156, t25168, t25191, t25192, t25193, t25194, t25205, t25206)
}
