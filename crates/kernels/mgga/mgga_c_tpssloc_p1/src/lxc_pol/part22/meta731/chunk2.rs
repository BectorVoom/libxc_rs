//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2399/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2399<F: Float>(t68673: F, t68693: F, t894: F, t901: F, t59759: F, t59761: F, t60308: F, t60310: F, t60312: F, t68638: F, t68640: F, t68643: F, t68646: F, t68649: F) -> (F, F, F) {
    let t68694 = t68673 + t68693;
    let t68695 = t894 * t68694;
    let t68697 = t901 * t68694;
    let t68699 = F::cast_from(0.247573125e0_f64) * t68638 + F::cast_from(0.247573125e0_f64) * t68640 - F::new(0.1294625e1) * t68643 + F::new(0.82524375e-1) * t68646 - F::new(0.82785e-1) * t68649 + F::new(0.181155e1) * t59759 - F::new(0.12077e1) * t59761 - F::new(0.33114e0) * t60308 + F::new(0.11038e0) * t60310 + F::cast_from(0.73586666666666666666e-1_f64) * t60312 + F::new(0.258925e1) * t68695 + F::new(0.16504875e0) * t68697;
    (t68695, t68697, t68699)
}
