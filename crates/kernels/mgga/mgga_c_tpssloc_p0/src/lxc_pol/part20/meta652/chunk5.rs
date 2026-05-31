//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2405/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2405<F: Float>(t10704: F, t2836: F, t49240: F, t912: F, t10655: F, t14422: F, t2793: F, t2842: F, t4396: F, t10662: F, t10702: F, t4399: F) -> (F, F, F, F) {
    let t49244 = F::cast_from(0.1551780387578202009e4_f64) * t49240 * t10704 * t2836 * t912;
    let t49256 = F::cast_from(18.0_f64) * t10655 * t14422;
    let t49259 = F::cast_from(18.0_f64) * t2842 * t4396 * t2793;
    let t49262 = F::cast_from(0.57895126195293126241e3_f64) * t10702 * t4399 * t10662;
    (t49244, t49256, t49259, t49262)
}
