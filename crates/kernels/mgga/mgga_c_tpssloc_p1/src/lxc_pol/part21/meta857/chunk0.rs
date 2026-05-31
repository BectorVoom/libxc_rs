//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3108/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3108<F: Float>(t43780: F, t43782: F, t43816: F, t43895: F, t50968: F, t50970: F, t50972: F, t50978: F, t51039: F, t51041: F, t64028: F, t64031: F, t64033: F, t64042: F, t64045: F) -> F {
    let t64389 = F::cast_from(0.16504875e0_f64) * t64028 - F::cast_from(0.16504875e0_f64) * t64031 + F::cast_from(0.776775e1_f64) * t64033 + F::cast_from(0.73586666666666666666e-1_f64) * t50968 + F::cast_from(0.36793333333333333333e-1_f64) * t50970 + F::cast_from(0.22076e0_f64) * t50972 + t43895 - F::cast_from(0.49057777777777777777e-1_f64) * t50978 + F::cast_from(0.13418888888888888889e0_f64) * t43780 + F::cast_from(0.26837777777777777778e0_f64) * t43782 - F::cast_from(0.62621481481481481482e0_f64) * t43816 - F::cast_from(0.258925e1_f64) * t64042 + F::cast_from(0.82785e-1_f64) * t64045 + F::cast_from(0.73586666666666666667e0_f64) * t51039 - F::cast_from(0.22076e0_f64) * t51041;
    t64389
}
