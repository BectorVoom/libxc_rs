//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1009/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1009<F: Float>(t7467: F, t8601: F, t4028: F, t8326: F, t7676: F, t1458: F, t31224: F, t33124: F, t33142: F, t33144: F, t33146: F, t33148: F, t8446: F, t5161: F, t8489: F, t1983: F) -> (F, F, F, F, F) {
    let t33150 = 4.0 * t8601 * t7467;
    let t33151 = t4028 * t8326;
    let t33152 = 2.0 * t33151;
    let t33153 = t7676 * t8326;
    let t33154 = 2.0 * t33153;
    let t33155 = 2.0 * t1458 * t31224 + t33124 + 4.0 * t33142 + 4.0 * t33144 + 4.0 * t33146 + t33148 + t33150 + t33152 + t33154 + t8446;
    let t33157 = t8489 * t5161;
    let t33158 = t1983 * t33157;
    (t33152, t33154, t33155, t33157, t33158)
}
