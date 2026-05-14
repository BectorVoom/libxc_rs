//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 359/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk359<F: Float>(t1519: F, t218: F, t1510: F, t860: F, t235: F, t1499: F, t226: F, t255: F, t812: F) -> (F, F, F, F) {
    let t1520 = t218 * t1519;
    let t1523 = t860 * t1510;
    let t1525 = t235 * t1519;
    let t1527 = t1499 * t255 - t1523 * t812 + t1525 * t226;
    (t1520, t1523, t1525, t1527)
}
