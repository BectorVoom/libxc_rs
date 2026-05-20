//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2707/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2707<F: Float>(t1427: F, t1434: F, t19326: F, t19405: F, t19441: F, t20210: F, t20265: F, t20285: F, t33: F, t3997: F, t3998: F, t4018: F, t5392: F, t5393: F, t5428: F, t5442: F, t629: F, t642: F, t66: F, t72: F, t75461: F, t75494: F, t75543: F, t80: F) -> F {
    let t75547 = -t5392 * t3997 * t80 / F::new(4.0) - t20210 * t642 / F::new(4.0) - t19326 * t1434 / F::new(4.0) - t5393 * t4018 / F::new(4.0) + t33 * (t75461 + t75494) * t80 / F::new(24.0) + t20265 * t642 / F::new(24.0) + t19405 * t1434 / F::new(8.0) + t5428 * t4018 / F::new(8.0) + t3998 * t5442 / F::new(8.0) + t1427 * t19441 / F::new(8.0) + t629 * t20285 / F::new(24.0) + t66 * t72 * t75543 / F::new(24.0);
    t75547
}
