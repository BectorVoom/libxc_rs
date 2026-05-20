//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1482/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1482<F: Float>(t5389: F, t5445: F, t1411: F, t1426: F, t1427: F, t1434: F, t19420: F, t19430: F, t20210: F, t20217: F, t20218: F, t20219: F, t20285: F, t2291: F, t2298: F, t31: F, t39096: F, t39114: F, t4007: F, t4012: F, t5392: F, t5393: F, t5398: F, t5403: F, t5427: F, t5428: F, t5442: F, t634: F, t638: F, t65: F, t66: F, t72: F, t75836: F, t75847: F, t75912: F, t80: F) -> (F, F, F) {
    let t79579 = t5389 * t5389;
    let t79585 = t5445 * t5445;
    let t79637 = -t5392 * t5427 * t80 / F::new(2.0) - t20210 * t1434 - t5393 * t5442 / F::new(2.0) - t5403 * t5442 - t1411 * t20285 / F::new(3.0) + t5428 * t5442 / F::new(4.0) + t1427 * t20285 / F::new(6.0) + t66 * t72 * (F::new(3640.0) / F::new(81.0) * t39096 * t75836 - F::new(560.0) / F::new(9.0) * t19420 * t5398 + F::new(28.0) / F::new(3.0) * t2291 * t75847 + F::new(112.0) / F::new(9.0) * t4007 * t20217 - F::new(4.0) / F::new(3.0) * t634 * t75912 + F::new(3640.0) / F::new(81.0) * t39114 * t75836 + F::new(560.0) / F::new(9.0) * t19430 * t5398 + F::new(28.0) / F::new(3.0) * t2298 * t75847 + F::new(112.0) / F::new(9.0) * t4012 * t20217 + F::new(4.0) / F::new(3.0) * t638 * t75912) / F::new(24.0) - t31 * t75912 * t65 * t80 / F::new(12.0) - t20218 * t1426 * t80 / F::new(3.0) - t20219 * t1434 / F::new(3.0);
    (t79579, t79585, t79637)
}
