//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 982/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk982<F: Float>(t1268: F, t26135: F, t1458: F, t22461: F, t24999: F, t26098: F, t26103: F, t26109: F, t26111: F, t26113: F, t26116: F, t26119: F, t26121: F, t26123: F, t26125: F, t4072: F, t6517: F, t671: F) -> (F, F) {
    let t26137 = 2.0 * t1268 * t26135;
    let t26138 = 2.0 * t1458 * t22461 + 2.0 * t1458 * t26103 + 2.0 * t24999 * t671 + 2.0 * t4072 * t6517 + t26098 + t26109 + t26111 + t26113 + t26116 + t26119 + t26121 + t26123 + t26125 + t26137;
    (t26137, t26138)
}
