//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 506/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk506<F: Float>(t2045: F, t77: F, t1986: F, t1994: F, t1997: F, t2026: F, t583: F, t603: F, t616: F, t71: F, t85: F) -> (F, F) {
    let t2046 = t77 * t2045;
    let t2049 = -t1986 * t85 / F::cast_from(12.0_f64) - t1994 * t85 / F::cast_from(12.0_f64) - t1997 * t85 / F::cast_from(6.0_f64) - t583 * t616 / F::cast_from(6.0_f64) + t2026 * t85 / F::cast_from(24.0_f64) + t603 * t616 / F::cast_from(12.0_f64) + t71 * t2046 / F::cast_from(24.0_f64);
    (t2046, t2049)
}
