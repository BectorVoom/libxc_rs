//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1514/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1514<F: Float>(t10481: F, t381: F, t360: F, t6739: F, t1057: F, t10960: F) -> (F, F, F, F) {
    let t11047 = t381 * t10481;
    let t11048 = t6739 * t360;
    let t11049 = t11047 * t11048;
    let t11051 = t10960 * t1057;
    (t11047, t11048, t11049, t11051)
}
