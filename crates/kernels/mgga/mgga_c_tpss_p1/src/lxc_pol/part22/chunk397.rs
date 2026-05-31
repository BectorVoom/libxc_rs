//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 397/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk397<F: Float>(t1289: F, t60: F, t1294: F, t1300: F, t44: F, t56: F, t601: F, t61: F, t38: F, t608: F, t612: F) -> (F, F, F, F, F) {
    let t1303 = t60 * t1289;
    let t1306 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t1294 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1300 * t61 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t1303 + t601;
    let t1307 = t38 * t1306;
    let t1310 = t608 * t1289;
    let t1311 = t612 * t1289;
    let t1313 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1310 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1311;
    (t1306, t1307, t1310, t1311, t1313)
}
