//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2395/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2395<F: Float>(t41959: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t60204: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F) -> F {
    let t68616 = F::cast_from(0.40256666666666666666e1_f64) * t68596 - F::cast_from(0.10064166666666666667e1_f64) * t68599 + F::cast_from(0.36231e1_f64) * t68602 - F::cast_from(0.10064166666666666667e1_f64) * t68605 - F::cast_from(0.543465e1_f64) * t68608 - F::cast_from(0.91983333333333333334e-1_f64) * t60204 - F::cast_from(0.60385000000000000002e0_f64) * t59663 + F::cast_from(0.20128333333333333334e0_f64) * t59665 + F::cast_from(0.30192500000000000001e0_f64) * t59680 + F::cast_from(0.80513333333333333334e0_f64) * t59688 - F::cast_from(0.40256666666666666668e0_f64) * t59694 + t41959;
    t68616
}
