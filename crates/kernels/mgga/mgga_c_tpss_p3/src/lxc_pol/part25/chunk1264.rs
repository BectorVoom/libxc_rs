//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1264/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1264<F: Float>(t21907: F, t485: F, t1795: F, t4637: F, t1165: F, t1338: F, t13565: F, t1799: F, t20289: F, t21180: F, t21227: F, t21786: F, t3493: F, t4674: F, t5801: F, t6234: F, t6323: F) -> (F, F, F) {
    let t21908 = t485 * t21907;
    let t21922 = t1795 * t4637;
    let t21944 = F::cast_from(2.0_f64) * t1165 * t21907 + F::cast_from(4.0_f64) * t1338 * t20289 + F::cast_from(2.0_f64) * t13565 * t1799 + F::cast_from(4.0_f64) * t1799 * t21180 + F::cast_from(2.0_f64) * t1799 * t21227 + F::cast_from(4.0_f64) * t3493 * t6323 + F::cast_from(2.0_f64) * t4674 * t5801 + F::cast_from(4.0_f64) * t6234 * t6323 + t21786 + F::cast_from(2.0_f64) * t21922;
    (t21908, t21922, t21944)
}
