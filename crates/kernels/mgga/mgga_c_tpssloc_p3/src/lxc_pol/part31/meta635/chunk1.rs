//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1900/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1900<F: Float>(t22633: F, t22635: F, t26337: F, t5353: F, t5325: F, t90488: F, t1307: F, t567: F, t6330: F, t90591: F, t28199: F, t6897: F, t794: F) -> (F, F, F, F) {
    let t97577 = t22633 * t22635 * t26337 * t5353;
    let t97583 = t22633 * t22635 * t90488 * t5325;
    let t97588 = t90591 * t22635 * t567 * t6330 * t1307;
    let t97599 = t6897 * t794 * t28199;
    (t97577, t97583, t97588, t97599)
}
