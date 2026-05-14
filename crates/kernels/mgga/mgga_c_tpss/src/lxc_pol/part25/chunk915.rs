//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 915/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk915<F: Float>(t4608: F, t582: F, t4573: F, t7737: F, t581: F, t3431: F, t3446: F, t2009: F, t4579: F, t13335: F, t48: F, t7750: F, t3455: F, t2016: F, t60: F, t1300: F, t3456: F, t3459: F, t44: F, t4589: F, t4592: F, t4597: F, t56: F, t589: F, t595: F, t7761: F) -> (F, F) {
    let t13365 = t582 * t4608;
    let t13370 = t7737 * t4573;
    let t13371 = t13370 * t581;
    let t13374 = t3446 * t3431;
    let t13379 = t2009 * t4579;
    let t13380 = t13379 * t581;
    let t13383 = t48 * t13335;
    let t13392 = t7750 * t4573;
    let t13393 = t13392 * t581;
    let t13396 = t3455 * t3431;
    let t13399 = t2016 * t4579;
    let t13400 = t13399 * t581;
    let t13403 = t60 * t13335;
    let t13406 = -20.0 / 27.0 * t589 * t4589 - 5.0 / 108.0 * t44 * t13371 + 5.0 / 9.0 * t44 * t13374 - 20.0 / 9.0 * t589 * t4592 + 5.0 / 18.0 * t44 * t13380 + 5.0 / 6.0 * t44 * t13383 - 220.0 / 27.0 * t4597 * t595 - 40.0 / 27.0 * t1300 * t3456 + 40.0 / 9.0 * t1300 * t3459 + 5.0 / 108.0 * t56 * t13393 + 5.0 / 9.0 * t56 * t13396 + 5.0 / 18.0 * t56 * t13400 - 5.0 / 6.0 * t56 * t13403 + t7761;
    (t13365, t13406)
}
