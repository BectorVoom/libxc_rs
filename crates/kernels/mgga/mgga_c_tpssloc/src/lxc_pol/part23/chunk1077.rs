//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1077/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1077<F: Float>(t11065: F, t42387: F, t10475: F, t42342: F, t42345: F, t2770: F, t283: F, t61: F, t1014: F, t42340: F, t42341: F, t23508: F, t360: F, t3127: F, t3131: F, t10474: F) -> (F, F, F, F, F, F, F, F) {
    let t43361 = t11065 * t42387;
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = 1.0 / t283 / t2770;
    let t43399 = t61 * t43398;
    let t43503 = t42340 * t42341 * t1014;
    let t43505 = t23508 * t360;
    let t43515 = t42340 * t42341 * t3127;
    let t43516 = t23508 * t3131;
    let t43553 = t42340 * t42341 * t10474;
    (t43361, t43385, t43399, t43503, t43505, t43515, t43516, t43553)
}
