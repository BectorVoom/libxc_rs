//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 527/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk527<F: Float>(t1985: F, t2009: F, t1992: F, t48: F, t59: F, t60: F, t234: F, t64: F, t2004: F, t44: F, t49: F, t56: F, t589: F, t592: F, t38: F, t45: F, t606: F) -> (F, F, F, F, F, F, F, F) {
    let t2010 = t2009 * t1985;
    let t2013 = t48 * t1992;
    let t2016 = 1.0 / t59;
    let t2017 = t2016 * t1985;
    let t2020 = t60 * t1992;
    let t2023 = t64 * t234;
    let t2024 = 88.0 / 9.0 * t2023;
    let t2025 = 88.0 / 9.0 * t2004 * t49 - 40.0 / 9.0 * t589 * t592 + 5.0 / 18.0 * t44 * t2010 + 5.0 / 6.0 * t44 * t2013 + 5.0 / 18.0 * t56 * t2017 - 5.0 / 6.0 * t56 * t2020 - t2024;
    let t2026 = t38 * t2025;
    let t2031 = t606 * t45;
    (t2016, t2017, t2020, t2023, t2024, t2025, t2026, t2031)
}
