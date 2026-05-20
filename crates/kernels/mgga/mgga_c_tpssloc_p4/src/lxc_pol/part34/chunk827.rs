//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 827/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk827<F: Float>(t1411: F, t1427: F, t1434: F, t19322: F, t20207: F, t20210: F, t20219: F, t20222: F, t20227: F, t20265: F, t20285: F, t5393: F, t5400: F, t5403: F, t5428: F, t5442: F, t66: F, t80: F) -> F {
    let t20288 = -t19322 * t20207 / F::new(4.0) - t20210 * t80 / F::new(4.0) - t5393 * t1434 / F::new(4.0) - t20219 * t80 / F::new(12.0) - t20222 * t80 / F::new(4.0) - t5400 * t1434 / F::new(4.0) - t20227 * t80 / F::new(4.0) - t5403 * t1434 / F::new(2.0) - t1411 * t5442 / F::new(4.0) + t20265 * t80 / F::new(24.0) + t5428 * t1434 / F::new(8.0) + t1427 * t5442 / F::new(8.0) + t66 * t20285 / F::new(24.0);
    t20288
}
