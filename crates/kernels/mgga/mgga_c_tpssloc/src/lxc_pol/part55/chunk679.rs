//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 679/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk679<F: Float>(t1198: F, t1218: F, t1232: F, t2134: F, t2136: F, t488: F, t7309: F, t7310: F, t7315: F, t7316: F, t7321: F, t7326: F, t7331: F, t7334: F, t7339: F, t7343: F, t7345: F) -> (F,) {
    let t7348 = t7309 - t7310 * t1198 / 288.0 + t7315 - 0.10093189023535097714e-3 * t7316 * t2136 - 0.10093189023535097714e-3 * t2134 * t7321 + 0.10093189023535097714e-3 * t7326 * t7331 + t7334 * t488 / 1536.0 + t7339 * t1218 / 1536.0 + t7343 - t7345 * t1232 / 2304.0;
    (t7348,)
}
