//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1161/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1161<F: Float>(t204: F, t376: F, t370: F, t374: F, t9697: F, t10473: F, t361: F, t363: F, t42342: F, t42345: F, t3131: F, t221: F, t339: F, t42813: F) -> (F, F, F, F, F, F) {
    let t43216 = t204 * t376;
    let t43253 = F::new(7.0) / F::new(31104.0) * t370 * t374 * t9697 * t376;
    let t43288 = F::new(1.0) / t10473 / t361;
    let t43291 = t42342 * t43288 * t363 * t42345;
    let t43292 = t3131 * t3131;
    let t43307 = F::new(5.0) / F::new(486.0) * t339 * t221 * t42813;
    (t43216, t43253, t43288, t43291, t43292, t43307)
}
