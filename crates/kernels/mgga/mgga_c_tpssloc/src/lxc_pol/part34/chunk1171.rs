//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1171/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1171<F: Float>(t20953: F, t6614: F, t20978: F, t23146: F, t20861: F, t2628: F, t6605: F, t6552: F, t7479: F, t98133: F, t1880: F, t21013: F, t214: F, t225: F, t258: F) -> (F, F, F, F, F) {
    let t105406 = t6614 * t20953;
    let t105412 = t23146 * t20978;
    let t105415 = t6605 * t2628 * t20861;
    let t105423 = t6552 * t98133 * t7479;
    let t105428 = t1880 * t214 * t21013 * t225 * t258;
    (t105406, t105412, t105415, t105423, t105428)
}
