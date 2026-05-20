//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3099/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3099<F: Float>(t43780: F, t43782: F, t43816: F, t44275: F, t50968: F, t50970: F, t50972: F, t50978: F, t51039: F, t51041: F, t64028: F, t64031: F, t64033: F, t64042: F, t64045: F) -> F {
    let t64212 = F::new(0.6311625e0) * t64028 - F::new(0.6311625e0) * t64031 + F::new(0.10589175e2) * t64033 + F::cast_from(0.9261777777777777778e-1_f64) * t50968 + F::cast_from(0.4630888888888888889e-1_f64) * t50970 + F::cast_from(0.27785333333333333334e0_f64) * t50972 + t44275 - F::cast_from(0.61745185185185185187e-1_f64) * t50978 + F::cast_from(0.22954444444444444444e0_f64) * t43780 + F::cast_from(0.45908888888888888888e0_f64) * t43782 - F::cast_from(0.10712074074074074074e1_f64) * t43816 - F::new(0.3529725e1) * t64042 + F::new(0.104195e0) * t64045 + F::cast_from(0.9261777777777777778e0_f64) * t51039 - F::cast_from(0.27785333333333333334e0_f64) * t51041;
    t64212
}
