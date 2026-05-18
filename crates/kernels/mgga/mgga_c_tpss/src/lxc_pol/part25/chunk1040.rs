//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1040/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1040<F: Float>(t10573: F, t10578: F, t10584: F, t10661: F, t10678: F, t10679: F, t10777: F, t10803: F, t14322: F, t14326: F, t14330: F, t14334: F, t14338: F, t2147: F, t2173: F, t3626: F, t8171: F, t8204: F, t8287: F) -> (F, F) {
    let t14343 = t10578 * t10584 * t10573;
    let t14347 = -F::new(5.0) / F::new(384.0) * t2173 * t14322 + t2173 * t14326 / F::new(384.0) - t8171 * t14330 / F::new(4.0) + t2147 * t14334 / F::new(8.0) + t2147 * t14338 / F::new(16.0) - t10661 + t10678 - F::new(119.0) / F::new(6912.0) * t10679 - t3626 * t14343 / F::new(192.0) - t8204 - F::new(119.0) / F::new(13824.0) * t8287 - t10777 - t10803;
    (t14343, t14347)
}
