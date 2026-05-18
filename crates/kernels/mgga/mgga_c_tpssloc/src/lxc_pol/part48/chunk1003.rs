//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1003/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1003<F: Float>(t31518: F, t652: F, t671: F, t8533: F, t9348: F, t23831: F, t7042: F, t23858: F, t8607: F, t26161: F, t31775: F, t92200: F) -> (F, F, F, F, F) {
    let t115672 = F::new(4.0) * t652 * t31518 * t671;
    let t115674 = F::new(2.0) * t9348 * t8533;
    let t115676 = F::new(2.0) * t7042 * t23831;
    let t115678 = F::new(2.0) * t8607 * t23858;
    let t115681 = F::new(4.0) * t26161 * t92200 * t31775;
    (t115672, t115674, t115676, t115678, t115681)
}
