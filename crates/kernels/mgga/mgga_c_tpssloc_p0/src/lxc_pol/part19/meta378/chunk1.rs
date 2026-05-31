//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1412/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1412<F: Float>(t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43872: F, t43875: F, t43882: F, t43884: F, t43887: F, t43890: F, t43892: F) -> F {
    let t43894 = -F::cast_from(0.18396666666666666667e0_f64) * t43855 - F::cast_from(0.98115555555555555555e-1_f64) * t43857 - F::cast_from(0.98115555555555555556e0_f64) * t43859 + F::cast_from(0.5519e0_f64) * t43861 + F::cast_from(0.11038e1_f64) * t43863 - F::cast_from(0.51785e1_f64) * t43866 + F::cast_from(0.3300975e0_f64) * t43869 + F::cast_from(0.11651625e2_f64) * t43872 - F::cast_from(0.247573125e0_f64) * t43875 - F::cast_from(0.485484375e1_f64) * t43882 + F::cast_from(0.258925e1_f64) * t43884 - F::cast_from(0.3883875e1_f64) * t43887 + F::cast_from(0.6189328125e-1_f64) * t43890 + F::cast_from(0.247573125e0_f64) * t43892;
    t43894
}
