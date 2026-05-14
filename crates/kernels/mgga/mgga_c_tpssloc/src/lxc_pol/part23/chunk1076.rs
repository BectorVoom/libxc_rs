//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1076/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1076<F: Float>(t370: F, t374: F, t376: F, t9697: F, t10473: F, t361: F, t363: F, t42342: F, t42345: F, t3131: F, t221: F, t339: F, t42813: F, t10216: F, t2978: F, t3061: F, t676: F) -> (F, F, F, F, F, F, F) {
    let t43253 = 7.0 / 31104.0 * t370 * t374 * t9697 * t376;
    let t43288 = 1.0 / t10473 / t361;
    let t43291 = t42342 * t43288 * t363 * t42345;
    let t43292 = t3131 * t3131;
    let t43307 = 5.0 / 486.0 * t339 * t221 * t42813;
    let t43317 = t2978 * t10216;
    let t43338 = t676 * t3061;
    (t43253, t43288, t43291, t43292, t43307, t43317, t43338)
}
