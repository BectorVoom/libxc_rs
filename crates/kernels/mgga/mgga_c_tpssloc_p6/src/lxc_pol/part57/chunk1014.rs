//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1014/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1014<F: Float>(t115262: F, t1983: F, t28826: F, t128393: F, t128397: F, t128401: F, t128404: F, t128406: F, t128413: F, t128415: F, t128418: F, t128420: F, t128422: F, t2036: F, t2039: F, t24999: F, t28811: F, t29211: F, t33133: F, t33204: F, t6517: F, t652: F, t7458: F, t7670: F, t7787: F, t7806: F, t7943: F) -> F {
    let t128429 = F::cast_from(6.0_f64) * t1983 * t115262 * t28826;
    let t128433 = -F::cast_from(2.0_f64) * t2039 * t28811 * t652 - t2036 * t28811 - F::cast_from(4.0_f64) * t24999 * t7806 - F::cast_from(2.0_f64) * t29211 * t6517 - F::cast_from(2.0_f64) * t33133 * t7943 - F::cast_from(4.0_f64) * t33204 * t7458 - F::cast_from(2.0_f64) * t7670 * t7787 + t128393 + t128397 - t128401 - t128404 - t128406 - t128413 - t128415 - t128418 - t128420 - t128422 + t128429;
    t128433
}
