//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 767/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk767<F: Float>(t332: F, t4597: F, t1297: F, t455: F, t52: F, t339: F, t454: F, t1128: F, t5072: F, t242: F, t5068: F, t5078: F, t5080: F, t5084: F, t5116: F, t5119: F, t5185: F, t5187: F, t5189: F, t5193: F, t5197: F, t5201: F) -> (F, F, F, F, F, F) {
    let t5223 = t4597 * t332;
    let t5229 = F::new(1.0) / t52 / t455 / t1297;
    let t5231 = t339 * t454 * t5229;
    let t5234 = t1128 * t5072;
    let t5235 = t242 * t5234;
    let t5238 = t1128 * t5068;
    let t5239 = t242 * t5238;
    let t5242 = -t5078 + t5080 - t5084 + t5116 + t5119 + t5185 + t5187 - t5189 + t5193 - t5197 - t5201;
    (t5223, t5229, t5231, t5235, t5239, t5242)
}
