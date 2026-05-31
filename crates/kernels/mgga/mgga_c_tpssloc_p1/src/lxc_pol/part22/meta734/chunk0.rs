//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2410/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2410<F: Float>(t59759: F, t59761: F, t60308: F, t60310: F, t60312: F, t68638: F, t68640: F, t68643: F, t68646: F, t68649: F, t68695: F, t68697: F) -> F {
    let t68877 = F::cast_from(0.46074375e0_f64) * t68638 + F::cast_from(0.46074375e0_f64) * t68640 - F::cast_from(0.9494625e0_f64) * t68643 + F::cast_from(0.15358125e0_f64) * t68646 - F::cast_from(0.82156666666666666667e-1_f64) * t68649 + F::cast_from(0.17938e1_f64) * t59759 - F::cast_from(0.11958666666666666667e1_f64) * t59761 - F::cast_from(0.32862666666666666666e0_f64) * t60308 + F::cast_from(0.10954222222222222222e0_f64) * t60310 + F::cast_from(0.73028148148148148146e-1_f64) * t60312 + F::cast_from(0.1898925e1_f64) * t68695 + F::cast_from(0.3071625e0_f64) * t68697;
    t68877
}
