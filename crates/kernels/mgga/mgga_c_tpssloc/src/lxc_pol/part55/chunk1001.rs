//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1001/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1001<F: Float>(t33118: F, t8513: F, t31047: F, t7687: F, t1983: F, t3701: F, t7752: F, t2019: F, t33094: F, t7467: F, t8601: F, t4028: F, t8326: F, t7676: F, t5161: F, t8489: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33119 = t8513 * t33118;
    let t33129 = t31047 * t7687;
    let t33131 = 3.0 * t1983 * t33129;
    let t33136 = t3701 * t7752;
    let t33137 = t2019 * t33136;
    let t33139 = 2.0 * t1983 * t33137;
    let t33148 = 2.0 * t33094;
    let t33150 = 4.0 * t8601 * t7467;
    let t33151 = t4028 * t8326;
    let t33152 = 2.0 * t33151;
    let t33153 = t7676 * t8326;
    let t33154 = 2.0 * t33153;
    let t33157 = t8489 * t5161;
    (t33119, t33129, t33131, t33136, t33137, t33139, t33148, t33150, t33152, t33154, t33157)
}
