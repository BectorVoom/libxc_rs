//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1111/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1111<F: Float>(t31537: F, t7467: F, t1873: F, t96361: F, t24999: F, t6534: F, t1458: F, t6514: F, t1868: F, t4072: F, t33085: F, t22461: F, t90400: F, t120112: F, t112594: F, t119815: F, t119820: F, t119996: F, t31224: F, t671: F) -> (F, F, F) {
    let t120140 = 4.0 * t31537 * t7467;
    let t120141 = t96361 * t1873;
    let t120143 = t24999 * t6534;
    let t120145 = t6514 * t1458;
    let t120146 = t120145 * t1873;
    let t120148 = t1868 * t4072;
    let t120149 = t120148 * t1873;
    let t120151 = t33085 * t6534;
    let t120153 = t22461 * t7467;
    let t120163 = t90400 * t1873;
    let t120165 = 2.0 * t120112;
    let t120166 = 2.0 * t112594 * t1458 + 2.0 * t119815 * t671 + 2.0 * t119820 * t1458 + 2.0 * t31224 * t4072 + t119996 + t120140 + 4.0 * t120141 + 4.0 * t120143 + 4.0 * t120146 + 4.0 * t120149 + 4.0 * t120151 + 4.0 * t120153 + 4.0 * t120163 + t120165;
    (t120145, t120148, t120166)
}
