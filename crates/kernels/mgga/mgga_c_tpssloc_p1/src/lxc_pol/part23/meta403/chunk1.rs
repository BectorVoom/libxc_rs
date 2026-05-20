//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1214/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1214<F: Float>(t1041: F, t248: F, t43338: F, t5677: F, t3070: F, t43198: F, t5908: F, t5884: F, t698: F, t973: F, t5889: F, t5893: F) -> (F, F, F, F, F) {
    let t62445 = t1041 * t248 * t43338 * t5677;
    let t62494 = t3070 * t43198 * t5908;
    let t62559 = t973 * t698 * t5884;
    let t62565 = t973 * t698 * t5889;
    let t62832 = t973 * t698 * t5893;
    (t62445, t62494, t62559, t62565, t62832)
}
