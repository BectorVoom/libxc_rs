//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 622/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk622<F: Float>(t2274: F, t5392: F, t5398: F, t55: F, t1420: F, t1423: F, t2282: F, t39: F, t51: F, t5408: F, t5411: F, t5416: F, t56: F, t33: F, t2291: F, t634: F) -> (F, F, F, F, F, F) {
    let t5421 = t2274 * t5392;
    let t5424 = t55 * t5398;
    let t5427 = 5.0 / 18.0 * t39 * t5408 + 5.0 / 6.0 * t39 * t5411 + 88.0 / 9.0 * t5416 * t56 + 40.0 / 9.0 * t1420 * t1423 + 5.0 / 18.0 * t51 * t5421 - 5.0 / 6.0 * t51 * t5424 - t2282;
    let t5428 = t33 * t5427;
    let t5433 = t2291 * t5392;
    let t5435 = t634 * t5398;
    (t5421, t5424, t5427, t5428, t5433, t5435)
}
