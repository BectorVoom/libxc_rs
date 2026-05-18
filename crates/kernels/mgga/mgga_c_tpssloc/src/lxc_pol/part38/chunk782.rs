//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 782/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk782<F: Float>(t232: F, t4233: F, t819: F, t820: F, t4180: F, t4181: F, t829: F, t120: F, t1484: F, t2645: F, t1516: F, t2697: F) -> (F, F, F, F, F, F) {
    let t4234 = t4233 * t232;
    let t4236 = t819 * t820 * t4234;
    let t4240 = t4180 * t4181 * t829;
    let t4248 = t120 * t1484;
    let t4250 = t2645 * t4248 * t829;
    let t4253 = t2697 * t1516;
    (t4234, t4236, t4240, t4248, t4250, t4253)
}
