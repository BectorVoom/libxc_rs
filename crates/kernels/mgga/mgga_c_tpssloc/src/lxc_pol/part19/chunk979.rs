//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 979/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk979<F: Float>(t28: F, t1081: F, t3672: F, t11122: F, t12001: F, t12072: F, t3231: F, t517: F, t12070: F, t157: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t12075 = t3672 * t1081;
    let t12081 = piecewise3(t29, 0.0, -8.0 / 27.0 * t12072 * t12001 + 4.0 / 3.0 * t12075 * t3231 + 4.0 / 3.0 * t517 * t11122);
    let t12083 = (t12070 + t12081) * t157;
    (t12075, t12083)
}
