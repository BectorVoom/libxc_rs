//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1103/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1103<F: Float>(t1983: F, t33137: F, t1873: F, t24999: F, t33085: F, t6517: F, t7467: F, t33094: F, t8601: F, t4028: F, t8326: F, t7676: F, t5161: F, t8489: F, t1799: F, t3701: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33139 = 2.0 * t1983 * t33137;
    let t33142 = t24999 * t1873;
    let t33144 = t33085 * t1873;
    let t33146 = t6517 * t7467;
    let t33148 = 2.0 * t33094;
    let t33150 = 4.0 * t8601 * t7467;
    let t33151 = t4028 * t8326;
    let t33152 = 2.0 * t33151;
    let t33153 = t7676 * t8326;
    let t33154 = 2.0 * t33153;
    let t33157 = t8489 * t5161;
    let t33158 = t1983 * t33157;
    let t33159 = t3701 * t1799;
    (t33139, t33142, t33144, t33146, t33148, t33150, t33152, t33154, t33157, t33158, t33159)
}
