//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 688/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk688<F: Float>(t1141: F, t1143: F, t1581: F, t220: F, t3124: F, t3138: F, t4293: F, t4303: F, t4307: F, t4310: F, t4314: F, t4317: F, t468: F, t1139: F, t1136: F, t1149: F, t1587: F, t3113: F, t4294: F, t4296: F, t4300: F, t473: F) -> (F, F, F) {
    let t4322 = t1141 * t1143 * t4307 + t1141 * t1143 * t4310 + t1141 * t1143 * t4317 + 2.0 * t1581 * t3124 * t4303 - t1581 * t3138 * t4314 + t220 * t4293 * t468;
    let t4323 = t1139 * t4322;
    let t4325 = 2.0 * t1136 * t4300 - t1136 * t4323 - t1149 * t4296 - t1587 * t3113 + t4294 * t473;
    (t4322, t4323, t4325)
}
