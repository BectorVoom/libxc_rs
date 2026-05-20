//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3075/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3075<F: Float>(t43780: F, t43782: F, t43816: F, t44348: F, t50952: F, t50954: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F) -> F {
    let t63811 = F::cast_from(0.79148148148148148147e-2_f64) * t50952 + F::cast_from(0.47488888888888888888e-1_f64) * t50954 + t44348 + F::cast_from(0.79148148148148148147e-2_f64) * t43780 + F::cast_from(0.15829629629629629629e-1_f64) * t43782 - F::cast_from(0.36935802469135802468e-1_f64) * t43816 + F::cast_from(0.17808333333333333333e-1_f64) * t63355 - F::cast_from(0.23744444444444444444e-1_f64) * t63359 + F::cast_from(0.15829629629629629629e-1_f64) * t63361 + F::cast_from(0.71233333333333333332e-1_f64) * t63365 - F::cast_from(0.71233333333333333332e-1_f64) * t63370 + F::cast_from(0.19787037037037037037e-1_f64) * t63374;
    t63811
}
