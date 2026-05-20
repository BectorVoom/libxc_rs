//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1184/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1184<F: Float>(t39970: F, t40010: F, t40062: F, t40101: F, t40147: F, t40204: F, t40303: F, t40450: F, t12167: F, t562: F, t12434: F, t1338: F) -> (F, F, F) {
    let t40453 = t39970 + t40010 + t40062 + t40101 + t40147 + t40204 + t40303 + t40450;
    let t40475 = t562 * t12167;
    let t40479 = t1338 * t12434;
    (t40453, t40475, t40479)
}
