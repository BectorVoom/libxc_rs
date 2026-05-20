//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3088/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3088<F: Float>(t43855: F, t43859: F, t43861: F, t43863: F, t44027: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t50952: F, t50954: F) -> F {
    let t64027 = t44027 - F::cast_from(0.30428395061728395062e-1_f64) * t43855 - F::cast_from(0.486854320987654321e0_f64) * t43859 + F::cast_from(0.91285185185185185187e-1_f64) * t43861 + F::cast_from(0.18257037037037037037e0_f64) * t43863 - F::cast_from(0.79724444444444444444e0_f64) * t50903 - F::cast_from(0.39862222222222222222e0_f64) * t50905 - F::cast_from(0.11958666666666666667e1_f64) * t50907 - F::cast_from(0.35433086419753086419e0_f64) * t50919 - F::cast_from(0.22145679012345679012e0_f64) * t50921 + F::cast_from(0.10629925925925925926e1_f64) * t50948 + F::cast_from(0.26574814814814814814e0_f64) * t50950 + F::cast_from(0.13287407407407407407e0_f64) * t50952 + F::cast_from(0.79724444444444444443e0_f64) * t50954;
    t64027
}
