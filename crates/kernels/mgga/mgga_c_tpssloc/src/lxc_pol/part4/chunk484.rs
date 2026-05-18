//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 484/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk484<F: Float>(t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t1731: F, t1737: F, t1744: F, t1748: F, t467: F, t488: F) -> F {
    let t1751 = -t1706 * t467 / F::new(36.0) + t1195 - t1174 * t1726 / F::new(288.0) + t1731 * t488 / F::new(3072.0) + t1213 * t1737 / F::new(3072.0) - t1744 * t488 / F::new(576.0) + t1224 - t1227 * t1748 / F::new(4608.0);
    t1751
}
