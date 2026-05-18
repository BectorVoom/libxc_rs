//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1305/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1305<F: Float>(t28821: F, t7756: F, t28823: F, t7685: F, t28835: F, t1983: F, t7687: F, t97817: F, t7688: F, t28860: F, t19451: F, t7468: F) -> (F, F, F, F, F, F, F) {
    let t105167 = F::new(3.0) * t28821 * t7756;
    let t105169 = F::new(6.0) * t7685 * t28823;
    let t105171 = F::new(9.0) * t7685 * t28835;
    let t105175 = F::new(9.0) * t1983 * t97817 * t7687;
    let t105177 = F::new(9.0) * t28821 * t7688;
    let t105179 = F::new(3.0) * t7685 * t28860;
    let t105181 = F::new(6.0) * t19451 * t7468;
    (t105167, t105169, t105171, t105175, t105177, t105179, t105181)
}
