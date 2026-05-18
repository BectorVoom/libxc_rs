//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 508/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk508<F: Float>(t2225: F, t594: F, t598: F, t15: F, t19: F, t601: F, t604: F, t84: F, t85: F, t24: F, t42: F, t54: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2226 = F::new(0.778e2) * t2225;
    let t2228 = F::new(0.16272e3) * t594 * t598;
    let t2229 = t15 * t15;
    let t2230 = F::new(1.0) / t2229;
    let t2232 = F::new(0.9492e2) * t19 * t2230;
    let t2235 = t601 * t604;
    let t2239 = F::new(1.0) / t85 / t84;
    let t2240 = t24 * t2239;
    let t2267 = F::new(1.0) / t42;
    let t2274 = F::new(1.0) / t54;
    (t2226, t2228, t2229, t2230, t2232, t2235, t2239, t2240, t2267, t2274)
}
