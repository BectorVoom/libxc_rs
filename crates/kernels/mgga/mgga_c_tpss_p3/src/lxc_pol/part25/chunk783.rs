//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 783/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk783<F: Float>(t219: F, t5392: F, t5395: F, t3319: F, t5371: F, t1228: F, t5366: F, t1634: F, t1636: F, t516: F, t518: F) -> (F, F, F, F) {
    let t5397 = (t5392 + t5395) * t219;
    let t5401 = t3319 * t5371;
    let t5404 = t1228 * t5366;
    let t5407 = F::new(6.0) * t1634 * t1636 - F::new(12.0) * t516 * t5401 + F::new(3.0) * t516 * t5404 - t518 * t5397;
    (t5397, t5401, t5404, t5407)
}
