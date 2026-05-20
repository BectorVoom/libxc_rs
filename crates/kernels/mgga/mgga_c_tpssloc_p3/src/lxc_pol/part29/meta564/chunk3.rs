//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1977/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1977<F: Float>(t1442: F, t1774: F, t2114: F, t25975: F, t25977: F, t25979: F, t25982: F, t25987: F, t25991: F, t25993: F, t25996: F, t25998: F, t26002: F, t26005: F, t27863: F, t5107: F, t672: F, t7264: F, t7408: F) -> F {
    let t27867 = -t1442 * t7408 - t1774 * t7264 - t2114 * t5107 - F::new(2.0) * t27863 * t672 - t25975 - t25977 - t25979 - t25982 + t25987 - t25991 - t25993 - t25996 - t25998 - t26002 - t26005;
    t27867
}
