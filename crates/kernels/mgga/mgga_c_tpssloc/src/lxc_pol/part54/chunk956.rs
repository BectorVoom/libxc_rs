//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 956/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk956<F: Float>(t1458: F, t6534: F, t3941: F, t1873: F, t4072: F, t3938: F, t7467: F, t671: F, t1401: F, t26135: F, t12461: F, t2094: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26542 = t6534 * t1458;
    let t26544 = 27.0 * t3941 * t26542;
    let t26545 = t1873 * t4072;
    let t26547 = 27.0 * t3941 * t26545;
    let t26549 = 0.135e2 * t3938 * t7467;
    let t26550 = t7467 * t671;
    let t26552 = 27.0 * t3941 * t26550;
    let t26554 = 0.135e2 * t1401 * t26135;
    let t26558 = t2094 * t12461;
    (t26542, t26544, t26545, t26547, t26549, t26550, t26552, t26554, t26558)
}
