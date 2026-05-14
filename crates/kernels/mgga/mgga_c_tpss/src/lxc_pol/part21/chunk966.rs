//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 966/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk966<F: Float>(t10353: F, t48: F, t1289: F, t1985: F, t7750: F, t2016: F, t3431: F, t581: F, t1992: F, t3455: F, t60: F, t10340: F, t10344: F, t10347: F, t1294: F, t1300: F, t2004: F, t2017: F, t2020: F, t3447: F, t3450: F, t44: F, t56: F, t589: F, t7761: F) -> (F,) {
    let t10354 = t48 * t10353;
    let t10362 = t7750 * t1289 * t1985;
    let t10365 = t2016 * t3431;
    let t10366 = t10365 * t581;
    let t10369 = t3455 * t1992;
    let t10372 = t60 * t10353;
    let t10375 = 220.0 / 27.0 * t2004 * t1294 - 40.0 / 27.0 * t589 * t3447 - 40.0 / 9.0 * t589 * t3450 - 5.0 / 108.0 * t44 * t10340 + 5.0 / 9.0 * t44 * t10344 + 5.0 / 18.0 * t44 * t10347 + 5.0 / 6.0 * t44 * t10354 - 20.0 / 27.0 * t1300 * t2017 + 20.0 / 9.0 * t1300 * t2020 + 5.0 / 108.0 * t56 * t10362 + 5.0 / 9.0 * t56 * t10366 + 5.0 / 18.0 * t56 * t10369 - 5.0 / 6.0 * t56 * t10372 + t7761;
    (t10375,)
}
