//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 932/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk932<F: Float>(t1458: F, t6534: F, t3941: F, t1873: F, t4072: F, t3938: F, t7467: F, t671: F, t1401: F, t26135: F, t23877: F, t23880: F, t26509: F, t26523: F, t26533: F, t26535: F, t26537: F, t26539: F, t26541: F, t5376: F, t577: F, t7010: F) -> (F, F, F, F) {
    let t26542 = t6534 * t1458;
    let t26544 = 27.0 * t3941 * t26542;
    let t26545 = t1873 * t4072;
    let t26547 = 27.0 * t3941 * t26545;
    let t26549 = 0.135e2 * t3938 * t7467;
    let t26550 = t7467 * t671;
    let t26552 = 27.0 * t3941 * t26550;
    let t26554 = 0.135e2 * t1401 * t26135;
    let t26555 = 0.45e1 * t26509 * t577 + 0.135e2 * t26523 * t671 + 0.135e2 * t23877 * t1458 + 27.0 * t23880 * t5376 + 0.135e2 * t7010 * t4072 + t26533 + t26535 + t26537 + t26539 + t26541 + t26544 + t26547 + t26549 + t26552 + t26554;
    (t26542, t26545, t26550, t26555)
}
