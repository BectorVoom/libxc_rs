//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3066/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3066<F: Float>(t43780: F, t43782: F, t43816: F, t43942: F, t50952: F, t50954: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F) -> F {
    let t63692 = F::cast_from(0.41203703703703703704e-2_f64) * t50952 + F::cast_from(0.24722222222222222223e-1_f64) * t50954 + t43942 + F::cast_from(0.41203703703703703703e-2_f64) * t43780 + F::cast_from(0.82407407407407407406e-2_f64) * t43782 - F::cast_from(0.19228395061728395061e-1_f64) * t43816 + F::cast_from(0.92708333333333333333e-2_f64) * t63355 - F::cast_from(0.12361111111111111111e-1_f64) * t63359 + F::cast_from(0.82407407407407407409e-2_f64) * t63361 + F::cast_from(0.37083333333333333334e-1_f64) * t63365 - F::cast_from(0.37083333333333333333e-1_f64) * t63370 + F::cast_from(0.10300925925925925926e-1_f64) * t63374;
    t63692
}
