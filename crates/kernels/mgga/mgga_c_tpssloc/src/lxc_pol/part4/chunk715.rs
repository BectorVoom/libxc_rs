//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 715/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk715<F: Float>(t31: F, t5398: F, t65: F, t1410: F, t1426: F, t2267: F, t5392: F, t43: F, t48: F, t480: F, t2274: F, t55: F, t1420: F, t1423: F, t2282: F, t39: F, t51: F, t56: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t5399 = t31 * t5398;
    let t5400 = t5399 * t65;
    let t5403 = t1410 * t1426;
    let t5408 = t2267 * t5392;
    let t5411 = t43 * t5398;
    let t5415 = 1.0 / t48 / t480;
    let t5416 = sigma2 * t5415;
    let t5421 = t2274 * t5392;
    let t5424 = t55 * t5398;
    let t5427 = 5.0 / 18.0 * t39 * t5408 + 5.0 / 6.0 * t39 * t5411 + 88.0 / 9.0 * t5416 * t56 + 40.0 / 9.0 * t1420 * t1423 + 5.0 / 18.0 * t51 * t5421 - 5.0 / 6.0 * t51 * t5424 - t2282;
    (t5399, t5400, t5403, t5408, t5411, t5416, t5427)
}
