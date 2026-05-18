//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 979/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk979<F: Float>(t1788: F, t2221: F, t225: F, t5213: F, t5211: F, t12248: F, t68: F, t544: F, t5230: F, t12189: F, t1804: F, t5194: F, t782: F) -> (F, F, F, F, F, F, F) {
    let t15984 = t2221 * t1788;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16046 = t68 * t12248;
    let t16047 = t544 * t16046;
    let t16060 = t5230 * t68;
    let t16078 = t12189 * t1804;
    let t16081 = t782 * t5194;
    (t15984, t16022, t16030, t16047, t16060, t16078, t16081)
}
