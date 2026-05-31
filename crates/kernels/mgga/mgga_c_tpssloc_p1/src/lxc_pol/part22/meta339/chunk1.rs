//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1535/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1535<F: Float>(t52: F, t5392: F, t638: F, t5398: F, t78: F, t16558: F, t3966: F, t4111: F, t607: F, t771: F, t16648: F, zeta_threshold: F) -> (F, F) {
    let t150 = t52 <= zeta_threshold;
    let t16649 = t638 * t5392;
    let t16654 = t78 * t5398;
    let t16660 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16649 * t607 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4111 * t3966 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16654 * t607 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t771 * t16558);
    let t16662 = t16648 / F::cast_from(2.0_f64) + t16660 / F::cast_from(2.0_f64);
    (t16649, t16662)
}
