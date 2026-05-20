//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 903/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk903<F: Float>(t21334: F, t291: F, t10608: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F) -> (F, F) {
    let t21336 = F::new(0.621814e-1) * t21334 * t291;
    let t21347 = -t10608 - F::cast_from(0.12361111111111111111e-1_f64) * t13598 + F::cast_from(0.61805555555555555556e-2_f64) * t17149 - F::cast_from(0.18541666666666666667e-1_f64) * t17165 + F::cast_from(0.92708333333333333334e-2_f64) * t17175 - F::cast_from(0.10300925925925925926e-1_f64) * t21147 + F::cast_from(0.37083333333333333333e-1_f64) * t21150 - F::cast_from(0.18541666666666666666e-1_f64) * t21124 - F::cast_from(0.55625000000000000001e-1_f64) * t21153 + F::cast_from(0.55625000000000000001e-1_f64) * t21128 - F::cast_from(0.92708333333333333333e-2_f64) * t21156;
    (t21336, t21347)
}
