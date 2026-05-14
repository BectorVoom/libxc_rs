//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 720/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk720<F: Float>(t3432: F, t70: F, t1290: F, t602: F, t1306: F, t582: F, t1289: F, t2009: F, t581: F, t3431: F, t48: F, t2016: F, t60: F, t1294: F, t1300: F, t2024: F, t44: F, t56: F, t589: F, t595: F) -> (F, F, F, F, F, F, F, F) {
    let t3433 = t3432 * t70;
    let t3436 = t1290 * t602;
    let t3441 = t582 * t1306;
    let t3446 = t2009 * t1289;
    let t3447 = t3446 * t581;
    let t3450 = t48 * t3431;
    let t3455 = t2016 * t1289;
    let t3456 = t3455 * t581;
    let t3459 = t60 * t3431;
    let t3462 = -20.0 / 9.0 * t589 * t1294 + 5.0 / 18.0 * t44 * t3447 + 5.0 / 6.0 * t44 * t3450 + 20.0 / 9.0 * t1300 * t595 + 5.0 / 18.0 * t56 * t3456 - 5.0 / 6.0 * t56 * t3459 - t2024;
    (t3433, t3436, t3441, t3446, t3447, t3450, t3455, t3462)
}
