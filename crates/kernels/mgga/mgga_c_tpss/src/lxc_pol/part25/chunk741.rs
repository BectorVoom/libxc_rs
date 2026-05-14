//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 741/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk741<F: Float>(t1139: F, t5294: F, t1136: F, t1587: F, t4296: F, t473: F, t5271: F, t5276: F, t1589: F, t1153: F, t198: F, t3154: F, t330: F, t5078: F, t5080: F, t5084: F, t5116: F, t5119: F, t5185: F, t5187: F, t5189: F, t5193: F, t5197: F, t5201: F) -> (F, F, F, F) {
    let t5295 = t1139 * t5294;
    let t5297 = 2.0 * t1136 * t5276 - t1136 * t5295 - 2.0 * t1587 * t4296 + t473 * t5271;
    let t5301 = t1589 * t1589;
    let t5305 = t1153 * t198 * t330 * t5297 - t198 * t3154 * t330 * t5301 - t5078 + t5080 - t5084 + t5116 + t5119 + t5185 + t5187 - t5189 + t5193 - t5197 - t5201;
    (t5295, t5297, t5301, t5305)
}
