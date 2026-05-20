//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2890/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2890<F: Float>(t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F, t60308: F, t60310: F, t60312: F, t60315: F, t60318: F, t60321: F, t60324: F, t60327: F) -> F {
    let t60329 = -F::new(0.181155e1) * t59748 + F::cast_from(0.40256666666666666666e1_f64) * t59753 - F::new(0.72462e1) * t59757 + F::new(0.12077e1) * t59759 - F::cast_from(0.80513333333333333333e0_f64) * t59761 - F::new(0.181155e1) * t59765 + F::new(0.12077e1) * t59769 - F::new(0.22076e0) * t60308 + F::cast_from(0.73586666666666666667e-1_f64) * t60310 + F::cast_from(0.49057777777777777778e-1_f64) * t60312 + F::new(0.16557e0) * t60315 + F::new(0.44152e0) * t60318 - F::new(0.5519e-1) * t60321 - F::cast_from(0.36793333333333333333e-1_f64) * t60324 - F::cast_from(0.8585111111111111111e-1_f64) * t60327;
    t60329
}
