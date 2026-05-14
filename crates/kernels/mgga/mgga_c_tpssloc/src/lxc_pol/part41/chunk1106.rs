//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1106/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1106<F: Float>(t5427: F, t608: F, t5392: F, t9287: F, t607: F, t3966: F, t3981: F, t2267: F, t5398: F, t16558: F, t43: F, t9300: F, t3990: F, t2274: F, t55: F, t1420: F, t39: F, t3991: F, t3994: F, t51: F, t5408: F, t5411: F, t5416: F, t615: F, t621: F, t9311: F) -> (F, F) {
    let t19363 = t608 * t5427;
    let t19368 = t9287 * t5392;
    let t19369 = t19368 * t607;
    let t19372 = t3981 * t3966;
    let t19377 = t2267 * t5398;
    let t19378 = t19377 * t607;
    let t19381 = t43 * t16558;
    let t19390 = t9300 * t5392;
    let t19391 = t19390 * t607;
    let t19394 = t3990 * t3966;
    let t19397 = t2274 * t5398;
    let t19398 = t19397 * t607;
    let t19401 = t55 * t16558;
    let t19404 = -20.0 / 27.0 * t615 * t5408 - 5.0 / 108.0 * t39 * t19369 + 5.0 / 9.0 * t39 * t19372 - 20.0 / 9.0 * t615 * t5411 + 5.0 / 18.0 * t39 * t19378 + 5.0 / 6.0 * t39 * t19381 - 220.0 / 27.0 * t5416 * t621 - 40.0 / 27.0 * t1420 * t3991 + 40.0 / 9.0 * t1420 * t3994 + 5.0 / 108.0 * t51 * t19391 + 5.0 / 9.0 * t51 * t19394 + 5.0 / 18.0 * t51 * t19398 - 5.0 / 6.0 * t51 * t19401 + t9311;
    (t19363, t19404)
}
