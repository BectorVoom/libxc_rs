//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1281/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1281<F: Float>(t27170: F, t8601: F, t120952: F, t2039: F, t102344: F, t1873: F, t27188: F, t6534: F, t121004: F, t121007: F, t33234: F, t23938: F, t7467: F, t26977: F, t26135: F, t7042: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t122727 = t8601 * t27170;
    let t122730 = t120952 * t2039;
    let t122731 = t102344 * t1873;
    let t122734 = t27188 * t6534;
    let t122735 = t121004 * t1873;
    let t122736 = t121007 * t1873;
    let t122737 = t33234 * t6534;
    let t122738 = t23938 * t7467;
    let t122739 = t26977 * t7467;
    let t122740 = t7042 * t26135;
    (t122727, t122730, t122731, t122734, t122735, t122736, t122737, t122738, t122739, t122740)
}
