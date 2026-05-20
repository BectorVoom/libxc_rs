//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1030/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1030<F: Float>(t1983: F, t23857: F, t8640: F, t115271: F, t115275: F, t115277: F, t115279: F, t115283: F, t115666: F, t115669: F, t115672: F, t115674: F, t115676: F, t115678: F, t115681: F, t1976: F, t23951: F, t24008: F, t24176: F, t31246: F, t7171: F, t8329: F, t8450: F) -> F {
    let t115684 = F::new(2.0) * t1983 * t8640 * t23857;
    let t115685 = -t1976 * t24008 - t23951 * t8450 + F::new(6.0) * t24176 * t8450 + F::new(6.0) * t31246 * t7171 - t115271 - t115275 - t115277 - t115279 + t115283 + t115666 - t115669 - t115672 - t115674 - t115676 + t115678 + t115681 + t115684 - t8329;
    t115685
}
