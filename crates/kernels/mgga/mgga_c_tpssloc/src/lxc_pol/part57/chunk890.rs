//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 890/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk890<F: Float>(t33617: F, t7458: F, t652: F, t7467: F, t7890: F, t33214: F, t7802: F, t29211: F, t8526: F, t115262: F, t1983: F, t28826: F, t128393: F, t128397: F, t128401: F, t128404: F, t128406: F, t128413: F, t2036: F, t2039: F, t24999: F, t28811: F, t33133: F, t33204: F, t6517: F, t7670: F, t7787: F, t7806: F, t7943: F) -> (F,) {
    let t128415 = 4.0 * t7458 * t33617;
    let t128418 = 4.0 * t652 * t7890 * t7467;
    let t128420 = 4.0 * t33214 * t7802;
    let t128422 = 2.0 * t8526 * t29211;
    let t128429 = 6.0 * t1983 * t115262 * t28826;
    let t128433 = -2.0 * t2039 * t28811 * t652 - t2036 * t28811 - 4.0 * t24999 * t7806 - 2.0 * t29211 * t6517 - 2.0 * t33133 * t7943 - 4.0 * t33204 * t7458 - 2.0 * t7670 * t7787 + t128393 + t128397 - t128401 - t128404 - t128406 - t128413 - t128415 - t128418 - t128420 - t128422 + t128429;
    (t128433,)
}
